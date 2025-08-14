/**
 * Copyright 2025 The VOID Authors. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
import {
  Decoration,
  DecorationSet,
  EditorView,
  WidgetType,
  ViewUpdate,
  keymap,
  ViewPlugin
} from '@codemirror/view';
import {
  StateField,
  EditorState,
  RangeSetBuilder,
  StateEffect,
  Extension,
  Facet,
  EditorSelection,
} from '@codemirror/state';
import { EditorView as NestedEditorView } from 'codemirror';
import { quotePlugin } from '../quote/quote';
import { inlinePlugin } from '../inline/inline';
import { combinedListPlugin } from '../lists/lists';
import { useSelectionStore } from '@/lib/logic/selectorStore';
import { hashtagField } from '../tags/tags';

const updateCalloutEffect = StateEffect.define<DecorationSet>();

interface CalloutData {
  from: number;
  to: number;
  headerFrom: number;
  headerTo: number;
  bodyFrom: number;
  bodyTo: number;
  tag: string;
  header: string;
  body: string;
}

let lastDocRef: any = null;
let cachedCallouts: CalloutData[] = [];

function parseCallouts(state: EditorState): CalloutData[] {
  if (lastDocRef === state.doc && cachedCallouts.length > 0) {
    return cachedCallouts;
  }

  const lines = state.doc.toString().split('\n');
  const result: CalloutData[] = [];
  const headerRegex = /^>\s*\[!(?<tag>[A-Z]+)\](?<header>.*)/;
  const bodyRegex = /^>\s(?!\[)(?<body>.*)/;

  for (let i = 0; i < lines.length;) {
    const headerMatch = lines[i].match(headerRegex);
    if (!headerMatch?.groups) { i++; continue; }

    const tag = headerMatch.groups.tag;
    const header = headerMatch.groups.header.trim();

    const fromLine = i;
    let toLine = i;

    const bodyLines: string[] = [];
    const bodyStartLine = i + 1;

    for (let j = i + 1; j < lines.length; j++) {
      const m = lines[j].match(bodyRegex);
      if (!m?.groups) break;
      bodyLines.push(m.groups.body);
      toLine = j;
    }

    const headerLine = state.doc.line(fromLine + 1);
    const lastLine = state.doc.line(toLine + 1);

    const from = headerLine.from;
    const to = lastLine.to;

    const bodyFrom = bodyLines.length
      ? state.doc.line(bodyStartLine + 1).from
      : headerLine.to;

    const bodyTo = bodyLines.length
      ? lastLine.to
      : headerLine.to;

    result.push({
      from, to,
      headerFrom: headerLine.from,
      headerTo: headerLine.to,
      bodyFrom, bodyTo,
      tag, header, body: bodyLines.join('\n')
    });

    i = toLine + 1;
  }

  lastDocRef = state.doc;
  cachedCallouts = result;
  return result;
}

class CalloutWidget extends WidgetType {
  private nestedView?: NestedEditorView;
  private rootEl?: HTMLElement;

  constructor(
    private readonly tag: string,
    private readonly header: string,
    private readonly body: string,
    private readonly outerView: EditorView
  ) {
    super();
  }

  eq(other: CalloutWidget): boolean {
    return this.tag === other.tag && this.header === other.header;
  }

  private findMyRange(): { from: number; to: number } | null {
    if (!this.rootEl) return null;
    const pos = this.outerView.posAtDOM(this.rootEl);
    if (pos < 0) return null;
    const found = parseCallouts(this.outerView.state).find(c => pos >= c.from && pos <= c.to);
    return found ? { from: found.from, to: found.to } : null;
  }

  toDOM(view: EditorView): HTMLElement {
    const line = document.createElement('div');
    line.className = 'wrapper';
    this.rootEl = line;

    const box = document.createElement('div');
    box.className = `cm-callout callout-${this.tag.toLowerCase()}`;

    const headerEl = document.createElement('div');
    headerEl.className = 'callout-header';
    headerEl.textContent = this.header || this.tag;

    const bodyEl = document.createElement('div');
    bodyEl.className = 'callout-body';

    if (!this.nestedView) {
      this.nestedView = new NestedEditorView({
        doc: this.body,
        parent: bodyEl,
        extensions: [
          IsNestedEditor.of(true),
          EditorView.lineWrapping,
          combinedListPlugin,
          calloutExtension,
          quotePlugin,
          inlinePlugin,
          hashtagField,
          NestedEditorView.editable.of(false),
          NestedEditorView.updateListener.of((update: ViewUpdate) => {
            if (!update.docChanged) return;
            const rng = this.findMyRange();
            if (!rng) return;

            const all = parseCallouts(this.outerView.state);
            const mine = all.find(c => c.from === rng.from && c.to === rng.to);
            if (!mine) return;

            const newBody = update.state.doc.toString();
            const prefixed = newBody.split('\n').map(l => `> ${l}`).join('\n');

            this.outerView.dispatch({
              changes: {
                from: mine.bodyFrom,
                to: mine.bodyTo,
                insert: prefixed
              }
            });
          })
        ]
      });

      requestAnimationFrame(() => {
        if (this.nestedView) {
          this.nestedView.dispatch({
            effects: updateCalloutEffect.of(
              buildCalloutDecorations(this.nestedView.state, this.nestedView)
            )
          });
        }
      });
    }

    if (!view.state.facet(IsNestedEditor)) {
      const editButton = document.createElement('div');
      editButton.className = 'callout-edit';
      editButton.addEventListener('mousedown', (e) => {
        e.preventDefault();
        e.stopPropagation();
      });
      editButton.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const rng = this.findMyRange();
        const anchor = rng ? (rng.to) : view.state.selection.main.head; // гарантированно внутрь
        view.focus();
        view.dispatch(view.state.update({
          selection: EditorSelection.cursor(anchor),
        }));
      });
      box.appendChild(editButton);
    }

    box.appendChild(headerEl);
    box.appendChild(bodyEl);
    box.style.overflowY = 'scroll';

    if (!view.state.facet(IsNestedEditor)) {
      const lines = this.body.split('\n');
      const nestedCount = lines.filter(a => a.includes('> [!')).length;
      let estimatedHeight =
        (lines.length + 2 + nestedCount) * view.defaultLineHeight +
        (nestedCount * 75);
      if (nestedCount === 0) {
        estimatedHeight += 40;
      }
      box.style.setProperty('height', `${estimatedHeight}px`, 'important');
    }

    line.appendChild(box);
    line.addEventListener('mousedown', (e) => {
      e.preventDefault();
    }, true);

    return line;
  }

  ignoreEvent(event: Event): boolean {
    const target = event.target as HTMLElement;
    return !!target.closest('.callout-edit');
  }

  destroy() {
    this.nestedView?.destroy();
    this.nestedView = undefined;
    this.rootEl = undefined;
  }
}


function buildCalloutDecorations(state: EditorState, view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const callouts = parseCallouts(state);
  const store = useSelectionStore();

  const rangeIntersects = (aFrom: number, aTo: number, bFrom: number, bTo: number) =>
    aFrom < bTo && bFrom < aTo;

  const selectionHitsRange = (from: number, to: number) => {
    for (const r of state.selection.ranges) {
      if (r.empty) {
        return false;
      } else {
        if (rangeIntersects(r.from, r.to, from, to)) return true;
      }
    }
    return false;
  };

  for (const { from, to, tag, header, body } of callouts) {
    const hits = selectionHitsRange(from, to);

    const sel = state.selection.main;
    const inside = sel.from >= from && sel.from <= to;
    const selecting = store.current === 'true';
    if ((!selecting && hits) || (inside && !selecting)) continue;

    const widget = new CalloutWidget(tag, header, body, view);
    builder.add(from, to, Decoration.replace({ widget, side: 1 }));
  }

  return builder.finish();
}

const calloutDecorationField = StateField.define<DecorationSet>({
  create() {
    return Decoration.none;
  },
  update(deco, tr) {
    for (const e of tr.effects) {
      if (e.is(updateCalloutEffect)) return e.value;
    }
    if (tr.docChanged) {
      return deco.map(tr.changes);
    }
    return deco;
  },
  provide(field) {
    return EditorView.decorations.from(field);
  }
});

function handleEnterForCallout(view: EditorView): boolean {
  const { state } = view;
  const { head } = state.selection.main;
  const line = state.doc.lineAt(head);
  const text = line.text;

  const match = text.match(/^((?:>\s*)+)(.*)/);
  if (!match) return false;

  const rawPrefix = match[1];
  const content = match[2];

  const depth = (rawPrefix.replace(/\s/g, '').match(/>/g) || []).length;
  const cleanPrefix = Array(depth).fill('> ').join('');

  if (content.trim() === '') {
    if (depth > 1) {
      const reducedPrefix = Array(depth - 1).fill('> ').join('');
      view.dispatch({
        changes: { from: line.from, to: line.to, insert: reducedPrefix },
        selection: { anchor: line.from + reducedPrefix.length },
        scrollIntoView: true
      });
    } else {
      view.dispatch({
        changes: { from: line.from, to: line.to, insert: '' },
        selection: { anchor: line.from },
        scrollIntoView: true
      });
    }
    return true;
  }

  const insertText = `\n${cleanPrefix}`;
  view.dispatch({
    changes: { from: head, to: head, insert: insertText },
    selection: { anchor: head + insertText.length },
    scrollIntoView: true
  });

  return true;
}

const calloutKeymap = keymap.of([
  { key: 'Enter', run: handleEnterForCallout }
]);

let updateTimeout: number | undefined;

const forceRecalcOnPointerUp = ViewPlugin.fromClass(class {
  private onUp = () => {
    const store = useSelectionStore();
    store.toggleFalse();
    const view = this.view;
    const decorations = buildCalloutDecorations(view.state, view);
    view.dispatch({ effects: updateCalloutEffect.of(decorations) });
  };

  constructor(private readonly view: EditorView) {
    const doc = view.dom.ownerDocument;
    doc.addEventListener('mouseup', this.onUp, true); // capture: ловим даже снаружи
  }

  destroy() {
    const doc = this.view.dom.ownerDocument;
    doc.removeEventListener('mouseup', this.onUp, true);
  }
});

export const calloutExtension: Extension = [
  calloutDecorationField,
  calloutKeymap,
  forceRecalcOnPointerUp,
  EditorView.updateListener.of((update) => {
    if (update.selectionSet && !update.docChanged) {
      const view = update.view;
      const decorations = buildCalloutDecorations(view.state, view);
      view.dispatch({ effects: updateCalloutEffect.of(decorations) });
      return;
    }

    if (!update.docChanged) return;

    let needsRebuild = false;
    const before = parseCallouts(update.startState);

    if (!before.length) {
      needsRebuild = true;
    } else {
      update.changes.iterChanges((fromA, toA) => {
        if (needsRebuild) return;
        const hitHeaderOrOutside = before.some(c =>
          (fromA <= c.headerTo && toA >= c.headerFrom) ||
          fromA <= c.from || toA >= c.to
        );
        if (hitHeaderOrOutside) needsRebuild = true;
      });
    }

    if (needsRebuild) {
      if (updateTimeout !== undefined) clearTimeout(updateTimeout);
      updateTimeout = window.setTimeout(() => {
        const view = update.view;
        const decorations = buildCalloutDecorations(view.state, view);
        view.dispatch({ effects: updateCalloutEffect.of(decorations) });
        updateTimeout = undefined;
      }, 10);
    }
  })
];
export const IsNestedEditor = Facet.define<boolean, boolean>({
  combine: values => values.length ? values[0] : false
});
