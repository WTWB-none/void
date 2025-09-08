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
  EditorView,
  ViewPlugin,
  ViewUpdate,
  WidgetType,
} from '@codemirror/view';
import { RangeSetBuilder } from '@codemirror/state';

type GetSet = { getName: () => string; setName: (n: string) => void };

class FilenameWidget extends WidgetType {
  constructor(private api: GetSet) { super(); }
  eq(other: FilenameWidget) { return this.api.getName() === other.api.getName(); }

  toDOM() {
    const wrap = document.createElement('div');
    wrap.className = 'cm-filetitle-wrap';
    wrap.style.display = 'block';

    const input = document.createElement('input');
    input.type = 'text';
    input.className = 'cm-filetitle outline-0';

    input.value = this.api.getName();
    input.spellcheck = false;
    input.autocomplete = 'off';
    input.autocapitalize = 'off';
    input.setAttribute('aria-label', 'Filename');

    input.addEventListener('keydown', (e) => e.stopPropagation());
    input.addEventListener('keyup', (e) => e.stopPropagation());
    input.addEventListener('keypress', (e) => e.stopPropagation());
    input.addEventListener('mousedown', (e) => e.stopPropagation());
    input.addEventListener('click', (e) => e.stopPropagation());
    input.addEventListener('focusout', () => this.api.setName(input.value))

    wrap.appendChild(input);
    return wrap;
  }

  ignoreEvent(_e: Event) { return true; }
}

export function filenameWidgetExt(api: GetSet) {
  return ViewPlugin.fromClass(class {
    decorations = this.build();
    constructor(_view: EditorView) { }

    update(u: ViewUpdate) {
      if (u.docChanged || u.viewportChanged || u.transactions.length) {
      }
      this.decorations = this.build();
    }

    build() {
      const b = new RangeSetBuilder<Decoration>();
      b.add(
        0,
        0,
        Decoration.widget({
          widget: new FilenameWidget(api),
          side: -1,
        })
      );
      return b.finish();
    }
  }, { decorations: v => v.decorations });
}
