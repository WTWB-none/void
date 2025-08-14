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
  ViewUpdate,
  WidgetType,
  ViewPlugin,
} from '@codemirror/view';
import {
  EditorState,
  RangeSetBuilder,
  StateField,
  StateEffect,
  Extension,
} from '@codemirror/state';
import { useSelectionStore } from '@/lib/logic/selectorStore';

function findPageBreaker(
  line: string,
  offset: number,
  cursor: number,
  state: EditorState
) {
  const decorations: { from: number; to: number }[] = [];
  const regexp = /^---\s*$/;

  const intersection = (aFrom: number, aTo: number, bFrom: number, bTo: number) =>
    aFrom < bTo && bFrom < aTo;

  const inside = cursor >= offset && cursor <= offset + line.length;

  const checkSelection = (from: number, to: number) => {
    for (let r of state.selection.ranges) {
      if (!r.empty && intersection(r.from, r.to, from, to)) return true;
    }
    return false;
  };

  const store = useSelectionStore();
  const selecting = store.current === 'true';
  const hits = checkSelection(offset, offset + line.length);

  if ((!selecting && hits) || (inside && !selecting)) return null;

  if (regexp.test(line)) {
    decorations.push({ from: offset, to: offset + line.length });
  }

  return decorations;
}

const updatePageBreakerEffect = StateEffect.define<DecorationSet>();

function buildPageBreakerDecorations(state: EditorState): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const doc = state.doc;
  const cursor = state.selection.main.head;

  if (doc.length === 0) return Decoration.none;

  for (let i = 1; i <= doc.lines; i++) {
    const line = doc.line(i);
    const matches = findPageBreaker(line.text, line.from, cursor, state);
    if (!matches) continue;
    for (const m of matches) {
      builder.add(
        m.from,
        m.to,
        Decoration.replace({ widget: new PageBreakerWidget(), inclusive: false, side: 1 })
      );
    }
  }

  return builder.finish();
}

const pageBreakerField = StateField.define<DecorationSet>({
  create() {
    return Decoration.none;
  },
  update(deco, tr) {
    for (const e of tr.effects) {
      if (e.is(updatePageBreakerEffect)) return e.value;
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

const forceRecalcOnPointerUp = ViewPlugin.fromClass(class {
  private onUp = () => {
    const store = useSelectionStore();
    store.toggleFalse(); // завершили выделение
    const view = this.view;
    const decorations = buildPageBreakerDecorations(view.state);
    view.dispatch({ effects: updatePageBreakerEffect.of(decorations) });
  };

  constructor(private readonly view: EditorView) {
    const doc = view.dom.ownerDocument;
    doc.addEventListener('mouseup', this.onUp, true);
  }

  destroy() {
    const doc = this.view.dom.ownerDocument;
    doc.removeEventListener('mouseup', this.onUp, true);
  }
});

export const pageBreaker: Extension = [
  pageBreakerField,
  forceRecalcOnPointerUp,
  EditorView.updateListener.of((update: ViewUpdate) => {
    if (update.selectionSet && !update.docChanged) {
      const view = update.view;
      const decorations = buildPageBreakerDecorations(view.state);
      view.dispatch({ effects: updatePageBreakerEffect.of(decorations) });
      return;
    }

    if (update.docChanged || update.viewportChanged) {
      const view = update.view;
      const decorations = buildPageBreakerDecorations(view.state);
      view.dispatch({ effects: updatePageBreakerEffect.of(decorations) });
    }
  })
];

class PageBreakerWidget extends WidgetType {
  toDOM(): HTMLElement {
    const el = document.createElement('div');
    el.classList.add('md-pagebreaker');
    return el;
  }

  ignoreEvent(): boolean {
    return false;
  }
}
