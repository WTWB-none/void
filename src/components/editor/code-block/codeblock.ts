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
  EditorSelection,
  EditorState,
  RangeSetBuilder,
  StateField,
  StateEffect,
} from '@codemirror/state';
import {
  Decoration,
  DecorationSet,
  EditorView,
  WidgetType,
  ViewPlugin,
} from '@codemirror/view';
import { codeToHtml } from 'shiki';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useSelectionStore } from '@/lib/logic/selectorStore';

const updateCodeblockEffect = StateEffect.define<DecorationSet>();

class CodeBlockWidget extends WidgetType {
  constructor(
    private readonly lang: string,
    private readonly code: string,
    private readonly from: number,
    private readonly to: number,
  ) {
    super();
  }
  toDOM(view: EditorView): HTMLElement {
    const el = document.createElement('div');
    el.className = 'cm-code';

    const copyButton = document.createElement('div');
    copyButton.className = 'cm-code-button';
    copyButton.style.display = 'none';
    copyButton.style.zIndex = '100';
    copyButton.innerHTML =
      "<svg viewBox='0 0 24 24' xmlns='http://www.w3.org/2000/svg'><g transform=\"matrix(1 0 0 1 12 12)\"><path style=\"stroke:none;fill:rgb(128,128,128);\" transform=\" translate(-12, -12)\" d=\"M 4 2 C 2.895 2 2 2.895 2 4 L 2 18 L 4 18 L 4 4 L 18 4 L 18 2 L 4 2 z M 8 6 C 6.895 6 6 6.895 6 8 L 6 20 C 6 21.105 6.895 22 8 22 L 20 22 C 21.105 22 22 21.105 22 20 L 22 8 C 22 6.895 21.105 6 20 6 L 8 6 z M 8 8 L 20 8 L 20 20 L 8 20 L 8 8 z\"/></g></svg>";

    const language = document.createElement('div');
    language.className = 'cm-code-lang';
    language.textContent = this.lang;

    const body = document.createElement('div');
    body.className = 'cm-code-body';

    el.addEventListener('mouseenter', () => {
      language.style.display = 'none';
      copyButton.style.display = '';
    });
    el.addEventListener('mouseleave', () => {
      language.style.display = '';
      copyButton.style.display = 'none';
    });

    copyButton.addEventListener('mousedown', (e) => {
      e.preventDefault();
      e.stopPropagation();
    }, true);
    copyButton.addEventListener('click', async (e) => {
      e.preventDefault();
      e.stopPropagation();
      try { await writeText(this.code); } catch { }
    }, true);

    body.addEventListener('mousedown', (e) => {
      e.preventDefault();
      e.stopPropagation();
      view.dispatch(view.state.update({
        selection: EditorSelection.cursor(this.from),
      }));
    }, true);

    let to = view.state.doc.lineAt(this.to).number;
    let from = view.state.doc.lineAt(this.from).number;
    const estimatedHeight = ((to - from) * view.defaultLineHeight) + 10;
    codeToHtml(this.code, {
      lang: this.lang.toLowerCase(),
      theme: 'catppuccin-mocha',
    }).then((v) => {
      body.innerHTML = v;
      const shiki = body.getElementsByClassName('shiki').item(0) as HTMLElement;
      const cm = getComputedStyle(view.contentDOM);
      const lineH = `${view.defaultLineHeight}px`;

      shiki.style.setProperty('font-family', cm.fontFamily, 'important');
      shiki.style.setProperty('font-size', cm.fontSize, 'important');
      shiki.style.setProperty('line-height', lineH, 'important');

      const codeEl = shiki.querySelector('code') as HTMLElement | null;
      if (codeEl) {
        codeEl.style.setProperty('font-family', cm.fontFamily, 'important');
        codeEl.style.setProperty('font-size', cm.fontSize, 'important');
        codeEl.style.setProperty('line-height', lineH, 'important');
      }
      body.style.setProperty('height', `${estimatedHeight}px`, 'important');
      el.appendChild(copyButton);
      el.appendChild(language);
      el.appendChild(body);
    }, null);
    el.style.setProperty('height', `${estimatedHeight}px`, 'important');
    return el;
  }
  ignoreEvent(event: Event): boolean {
    const target = event.target as HTMLElement;
    return !!target.closest('.cm-code-button') || !!target.closest('.cm-code-body');
  }
}

function parseCodeblock(state: EditorState): DecorationSet {
  const doc = state.doc;
  const decoration = new RangeSetBuilder<Decoration>();

  let lang = '';
  let code = '';
  let from = 1;
  let to = 1;

  const headerRegexp = /```(?<lang>[^\s`]+)?\s*$/;
  const closeRegexp = /^```\s*$/;

  const intersects = (aFrom: number, aTo: number, bFrom: number, bTo: number) =>
    aFrom < bTo && bFrom < aTo;

  const selectionHitsRange = (from: number, to: number) => {
    for (const r of state.selection.ranges) {
      if (r.empty) {
        if (r.from >= from && r.from <= to) return true; // каретка внутри
      } else {
        if (intersects(r.from, r.to, from, to)) return true;
      }
    }
    return false;
  };

  const store = useSelectionStore();
  const selecting = store.current === true || store.current === 'true';

  for (let i = 1; i <= doc.lines; i++) {
    code = '';
    const headerLine = doc.line(i);
    const match = headerLine.text.match(headerRegexp);
    if (!match) continue;

    lang = (match.groups?.lang ?? 'text').trim() || 'text';
    from = headerLine.from;

    let closed = false;
    for (let j = i + 1; j <= doc.lines; j++) {
      const ln = doc.line(j);
      if (closeRegexp.test(ln.text)) {
        to = ln.to;
        i = j;
        closed = true;
        break;
      }
      code += ln.text + '\n';
    }
    if (!closed || to <= from) {
      continue;
    }

    const hits = selectionHitsRange(from, to);
    const showMd = !selecting && hits;
    if (!showMd) {
      decoration.add(from, to, Decoration.replace({
        widget: new CodeBlockWidget(lang, code, from, to),
        side: 1
      }));
    }

    lang = '';
    code = '';
    from = 1;
    to = 1;
  }

  return decoration.finish();
}

const forceRecalcOnPointerUp = ViewPlugin.fromClass(class {
  private onUp = () => {
    const view = this.view;
    const store = useSelectionStore();
    if (store.toggleFalse) store.toggleFalse();

    const win = view.dom.ownerDocument.defaultView!;
    win.requestAnimationFrame(() => {
      const decorations = parseCodeblock(view.state);
      view.dispatch({ effects: updateCodeblockEffect.of(decorations) });
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

export const CodeBlockExtension = StateField.define<DecorationSet>({
  create: parseCodeblock,
  update(deco, tr) {
    for (const e of tr.effects) {
      if (e.is(updateCodeblockEffect)) return e.value;
    }
    if (tr.docChanged || tr.selection) {
      return parseCodeblock(tr.state);
    }
    return deco;
  },
  provide: f => EditorView.decorations.from(f)
});

export const CodeBlockPlugin = [CodeBlockExtension, forceRecalcOnPointerUp];
