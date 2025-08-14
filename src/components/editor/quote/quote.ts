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
  keymap,
  ViewPlugin,
} from '@codemirror/view';
import {
  StateField,
  EditorState,
  RangeSetBuilder,
  EditorSelection,
  StateEffect,
} from '@codemirror/state';
import { useSelectionStore } from '@/lib/logic/selectorStore';

const updateQuoteEffect = StateEffect.define<DecorationSet>();

class QuoteWidget extends WidgetType {
  constructor(private readonly body: string, private readonly to: number) {
    super();
  }
  toDOM(view: EditorView): HTMLElement {
    const block = document.createElement('blockquote');
    block.className = 'quote-block';
    block.textContent = this.body;

    block.addEventListener('click', (e) => {
      e.preventDefault();
      e.stopPropagation();
      view.focus();
      view.dispatch(view.state.update({
        selection: EditorSelection.cursor(this.to),
      }));
    }, true);

    return block;
  }
  ignoreEvent(): boolean { return true; }
}

function parseQuotes(state: EditorState): Array<{ from: number; to: number; body: string; }> {
  const lines = state.doc.toString().split('\n');
  const result: Array<{ from: number; to: number; body: string; }> = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];

    const m = line.match(/^>\s?(.*)/);
    if (!m) { i++; continue; }

    const bodyParts: string[] = [m[1]];
    const fromLine = i;
    let toLine = i;

    for (let j = i + 1; j < lines.length; j++) {
      const next = lines[j];

      if (/^>\s*\[!/.test(next)) break;

      const mm = next.match(/^>\s?(.*)/);
      if (!mm) break;
      bodyParts.push(mm[1]);
      toLine = j;
    }

    const from = state.doc.line(fromLine + 1).from;
    const to = state.doc.line(toLine + 1).to;

    result.push({ from, to, body: bodyParts.join('\n') });
    i = toLine + 1;
  }

  return result;
}

const intersects = (aFrom: number, aTo: number, bFrom: number, bTo: number) =>
  aFrom < bTo && bFrom < aTo;

const selectionHitsRange = (state: EditorState, from: number, to: number) => {
  for (const r of state.selection.ranges) {
    if (r.empty) {
      if (r.from >= from && r.from <= to) return true;
    } else {
      if (intersects(r.from, r.to, from, to)) return true;
    }
  }
  return false;
};

function buildQuoteDecorations(state: EditorState): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const quotes = parseQuotes(state);

  const store = useSelectionStore();
  const selecting = store.current === true || store.current === 'true';

  for (const { from, to, body } of quotes) {
    const hits = selectionHitsRange(state, from, to);
    const showMd = !selecting && hits;
    if (showMd) continue;

    builder.add(from, to, Decoration.replace({
      widget: new QuoteWidget(body, to),
      side: 1
    }));
  }

  return builder.finish();
}

const forceRecalcOnPointerUp = ViewPlugin.fromClass(class {
  private onUp = () => {
    const store = useSelectionStore();
    if (store.toggleFalse) store.toggleFalse();

    const view = this.view;
    const win = view.dom.ownerDocument.defaultView!;
    win.requestAnimationFrame(() => {
      const decos = buildQuoteDecorations(view.state);
      view.dispatch({ effects: updateQuoteEffect.of(decos) });
    });
  };

  constructor(private readonly view: EditorView) {
    const doc = view.dom.ownerDocument;
    doc.addEventListener('pointerup', this.onUp, true); // capture: ловим и вне редактора
  }

  destroy() {
    const doc = this.view.dom.ownerDocument;
    doc.removeEventListener('pointerup', this.onUp, true);
  }
});

const continueQuoteOnEnter = keymap.of([{
  key: 'Enter',
  run(view) {
    const { state, dispatch } = view;
    const { from } = state.selection.main;
    const line = state.doc.lineAt(from);
    const text = line.text;

    if (/^>\s*\[!/.test(text)) return false;

    if (!/^>\s?/.test(text)) return false;

    if (/^>\s*$/.test(text.trim())) {
      dispatch(state.update({
        changes: { from: line.from, to: line.to, insert: '' },
        selection: { anchor: line.from },
        userEvent: 'input',
        scrollIntoView: true
      }));
      return true;
    }

    const prefixMatch = text.match(/^(\s*> ?)/);
    const prefix = prefixMatch?.[1] ?? '> ';
    dispatch(state.update({
      changes: { from, to: from, insert: `\n${prefix}` },
      selection: { anchor: from + 1 + prefix.length },
      userEvent: 'input',
      scrollIntoView: true
    }));
    return true;
  }
}]);

export const quotePlugin = [
  StateField.define<DecorationSet>({
    create: buildQuoteDecorations,
    update(deco, tr) {
      for (const e of tr.effects) {
        if (e.is(updateQuoteEffect)) return e.value;
      }
      if (tr.docChanged || tr.selection) {
        return buildQuoteDecorations(tr.state);
      }
      return deco;
    },
    provide: f => EditorView.decorations.from(f)
  }),
  forceRecalcOnPointerUp,
  continueQuoteOnEnter
];
