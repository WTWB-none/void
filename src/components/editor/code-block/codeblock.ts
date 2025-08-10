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
import { EditorSelection, EditorState, RangeSetBuilder, StateField } from '@codemirror/state';
import { Decoration, DecorationSet, EditorView, WidgetType } from '@codemirror/view';
import { codeToHtml } from 'shiki';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

class CodeBlockWidget extends WidgetType {
  constructor(private readonly lang: string, private readonly code: string, private readonly from: number) {
    super()
  }
  toDOM(view: EditorView): HTMLElement {
    let el = document.createElement('div');
    let copyButton = document.createElement('div');
    let language = document.createElement('div');
    let body = document.createElement('div');
    body.className = 'cm-code-body';
    language.textContent = this.lang;
    language.className = 'cm-code-lang';
    copyButton.className = 'cm-code-button';
    copyButton.style.display = 'none';
    copyButton.style.zIndex = '100';
    copyButton.innerHTML = "<svg viewBox='0 0 24 24' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink'><rect stroke='none' fill='#ffffff' opacity='0'/><g transform=\"matrix(1 0 0 1 12 12)\" ><path style=\"stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; fill: rgb(128,128,128); fill-rule: nonzero; opacity: 1;\" transform=\" translate(-12, -12)\" d=\"M 4 2 C 2.895 2 2 2.895 2 4 L 2 18 L 4 18 L 4 4 L 18 4 L 18 2 L 4 2 z M 8 6 C 6.895 6 6 6.895 6 8 L 6 20 C 6 21.105 6.895 22 8 22 L 20 22 C 21.105 22 22 21.105 22 20 L 22 8 C 22 6.895 21.105 6 20 6 L 8 6 z M 8 8 L 20 8 L 20 20 L 8 20 L 8 8 z\" stroke-linecap=\"round\" /></g></svg>"
    copyButton.addEventListener('click', (ev) => {
      ev.preventDefault();
      ev.stopPropagation();
      writeText(this.code);
    });

    copyButton.addEventListener('mousedown', (e) => {
      e.preventDefault();        // не даём браузеру трогать selection
      e.stopPropagation();
    });
    copyButton.addEventListener('mouseup', (e) => {
      e.preventDefault();
      e.stopPropagation();
    });
    copyButton.addEventListener('dragstart', (e) => {
      e.preventDefault();
      e.stopPropagation();
    });
    el.addEventListener('mouseenter', () => {
      language.style.display = 'none';
      copyButton.style.display = '';
    });
    el.addEventListener('mouseleave', () => {
      language.style.display = '';
      copyButton.style.display = 'none';
    });

    body.addEventListener('click', () => {
      view.dispatch(view.state.update({
        selection: EditorSelection.cursor(this.from),
      }))
      console.log('done');
    });
    codeToHtml(this.code, {
      lang: this.lang.toLowerCase(),
      theme: 'catppuccin-mocha',
    }).then((v) => {
      body.innerHTML = v;
      el.appendChild(copyButton);
      el.appendChild(language);
      el.appendChild(body);
    }, null);
    el.className = 'cm-code';
    return el;
  }
  ignoreEvent(event: Event): boolean {
    const target = event.target as HTMLElement;
    return !!target.closest('.cm-code-button') || !!target.closest('.cm-code-body');
  }
}

function parseCodeblock(state: EditorState): DecorationSet {
  const doc = state.doc;
  let lang = '';
  let code = '';
  let from = 1;
  let to = 1;
  let decoration = new RangeSetBuilder<Decoration>();
  const headerRegexp = /```(?<lang>\w+)/;
  const closeRegexp = /```/;
  for (let i = 1; i <= doc.lines; i++) {
    code = '';
    let match = doc.line(i).text.match(headerRegexp);
    if (match == null || match.groups == undefined) {
      continue;
    }
    else {
      lang = match.groups.lang
      from = doc.line(i).from;
      for (let j = i + 1; j <= doc.lines; j++) {
        let closeMatch = doc.line(j).text.match(closeRegexp);
        if (closeMatch == null) {
          code += doc.line(j).text + '\n';
          continue;
        }
        to = doc.line(j).to;
        i = j;
        break;
      }

      if (to > from && (state.selection.main.head < from || state.selection.main.head > to)) {
        if (state.selection.main.head < doc.lineAt(from - 1).from || state.selection.main.head > doc.lineAt(to + 1).to) {
          decoration.add(
            from,
            to,
            Decoration.replace(
              {
                widget: new CodeBlockWidget(lang, code, from),
                side: 1
              }
            )
          );
        }
        else {
          decoration.add(
            from,
            to,
            Decoration.replace(
              {
                widget: new CodeBlockWidget(lang, code, from),
                side: 1
              }
            )
          );
        }
        lang = '';
        code = '';
        from = 1;
        to = 1;
      }
    }
  }
  return decoration.finish();
}

export const CodeBlockPlugin = StateField.define<DecorationSet>({
  create: parseCodeblock,
  update(deco, tr) {
    if (tr.docChanged || tr.selection) {
      return parseCodeblock(tr.state);
    }
    return deco;
  },
  provide: f => EditorView.decorations.from(f)
})
