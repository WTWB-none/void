/*
Copyright 2025 The VOID Authors. All Rights Reserved.

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/
import {
  Decoration,
  DecorationSet,
  EditorView,
  WidgetType,
  ViewUpdate,
  keymap,
} from '@codemirror/view';
import {
  StateField,
  EditorState,
  RangeSetBuilder,
  StateEffect,
  Extension,
  Facet,
  EditorSelection
} from '@codemirror/state';
import { EditorView as NestedEditorView } from 'codemirror';
import { quotePlugin } from '../quote/quote';
import { inlinePlugin } from '../inline/inline';
import { combinedListPlugin } from '../lists/lists';
import { hashtagField } from '../tags/tags';

const updateCalloutEffect = StateEffect.define<DecorationSet>();

interface CalloutData {
  from: number;
  to: number;
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
    const bodyLines: string[] = [];
    const fromLine = i;
    let toLine = i;

    for (let j = i + 1; j < lines.length; j++) {
      const bodyMatch = lines[j].match(bodyRegex);
      if (!bodyMatch?.groups) break;
      bodyLines.push(bodyMatch.groups.body);
      toLine = j;
    }

    const from = state.doc.line(fromLine + 1).from;
    const to = state.doc.line(toLine + 1).to;

    result.push({ from, to, tag, header, body: bodyLines.join('\n') });
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
    return this.tag === other.tag && this.header === other.header && this.body === other.body;
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
    line.className = 'cm-line';
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
            const newText = update.state.doc.toString();
            this.outerView.dispatch({
              changes: {
                from: rng.from,
                to: rng.to,
                insert:
                  `> [!${this.tag}] ${this.header}\n` +
                  newText.split('\n').map(l => `> ${l}`).join('\n')
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
      editButton.addEventListener('click', () => {
        const rng = this.findMyRange();
        const anchor = rng ? rng.to : view.state.selection.main.head;
        view.dispatch(view.state.update({
          selection: EditorSelection.cursor(anchor),
          scrollIntoView: true,
        }));
      });
      box.appendChild(editButton);
    }

    box.appendChild(headerEl);
    box.appendChild(bodyEl);
    box.style.overflow = 'hidden';

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
  const sel = state.selection.main;

  for (const { from, to, tag, header, body } of callouts) {
    const inside = sel.from >= from && sel.from <= to;
    if (inside) continue;

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

export const calloutExtension: Extension = [
  calloutDecorationField,
  calloutKeymap,
  EditorView.updateListener.of((update) => {
    if (update.docChanged || update.selectionSet || update.viewportChanged) {
      if (updateTimeout !== undefined) {
        clearTimeout(updateTimeout);
      }

      if (update.selectionSet && !update.docChanged) {
        const view = update.view;
        const decorations = buildCalloutDecorations(view.state, view);
        view.dispatch({ effects: updateCalloutEffect.of(decorations) });
        return;
      }

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
