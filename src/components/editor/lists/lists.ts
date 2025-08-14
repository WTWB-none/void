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
  ViewPlugin,
  ViewUpdate,
  WidgetType,
  keymap
} from '@codemirror/view';
import {
  RangeSetBuilder,
  Extension,
} from '@codemirror/state';
import { insertNewlineAndIndent, indentMore, indentLess } from '@codemirror/commands';
import { IsNestedEditor } from '../callout/callout.ts'

class TodoCheckboxInline extends WidgetType {
  constructor(
    private readonly checked: boolean,
    private readonly onToggle: () => void
  ) {
    super();
  }

  toDOM(): HTMLElement {
    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.checked = this.checked;
    checkbox.className = 'cm-todo-checkbox';
    checkbox.onclick = this.onToggle;
    return checkbox;
  }
}

class BulletWidget extends WidgetType {
  constructor() {
    super();
  }

  toDOM(): HTMLElement {
    const bullet = document.createElement('span');
    bullet.className = 'cm-bullet-point';
    bullet.textContent = '•';
    bullet.style.color = '#666';
    bullet.style.fontWeight = 'bold';
    bullet.style.marginRight = '0.5em';
    return bullet;
  }
}

const listExtension = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = this.build(view);
    }

    update(update: ViewUpdate) {
      if (update.docChanged || update.selectionSet || update.viewportChanged) {
        this.decorations = this.build(update.view);
      }
    }

    build(view: EditorView): DecorationSet {
      const builder = new RangeSetBuilder<Decoration>();
      const cursorPos = view.state.selection.main.head;

      for (const { from, to } of view.visibleRanges) {
        let pos = from;

        while (pos <= to) {
          const line = view.state.doc.lineAt(pos);

          const todoMatch = /([-*]\s\[( |x)])\s/.exec(line.text);
          if (todoMatch && todoMatch.index !== undefined) {
            const checked = todoMatch[2] === 'x';
            const offset = todoMatch.index;
            const prefixStart = line.from + offset;
            const prefixEnd = prefixStart + todoMatch[1].length;
            const cursorInside = cursorPos >= prefixStart && cursorPos <= prefixEnd && !view.state.facet(IsNestedEditor);

            if (!cursorInside) {
              builder.add(
                prefixStart,
                prefixEnd,
                Decoration.replace({
                  widget: new TodoCheckboxInline(checked, () => {
                    const doc = view.state.doc;
                    const oldText = doc.sliceString(prefixStart, prefixEnd);
                    const newText = oldText.replace(
                      checked ? '[x]' : '[ ]',
                      checked ? '[ ]' : '[x]'
                    );
                    view.dispatch({
                      changes: {
                        from: prefixStart,
                        to: prefixEnd,
                        insert: newText
                      }
                    });
                  }),
                  inclusive: false
                })
              );
            }
          } else {
            const bulletMatch = /([ \t]*)-\s/.exec(line.text);
            if (bulletMatch && bulletMatch.index !== undefined) {
              const indent = bulletMatch[1];
              const restOfLine = line.text.substring(bulletMatch[0].length);

              if (!restOfLine.match(/^\[[ x]\]/)) {
                const bulletStart = line.from + indent.length;
                const bulletEnd = bulletStart + 1;
                const cursorInside = cursorPos >= bulletStart && cursorPos <= bulletEnd;

                if (!cursorInside) {
                  builder.add(
                    bulletStart,
                    bulletEnd,
                    Decoration.replace({
                      widget: new BulletWidget(),
                      inclusive: false
                    })
                  );
                } else {
                }
              } else {
              }
            } else {
            }
          }

          pos = line.to + 1;
        }
      }

      return builder.finish();
    }
  },
  {
    decorations: v => v.decorations
  }
);

function handleEnter(view: EditorView): boolean {
  const { state } = view;
  const { head } = state.selection.main;
  const line = state.doc.lineAt(head);

  const todoMatch = /^([ \t]*)([-*]\s\[[ x]])\s(.*)$/.exec(line.text);
  if (todoMatch) {
    const indent = todoMatch[1];
    const prefix = todoMatch[2];
    const content = todoMatch[3];

    if (content.trim() === '') {
      if (indent.length > 0) {
        return indentLess(view);
      } else {
        const removeEnd = line.from + todoMatch[0].length;
        view.dispatch({
          changes: {
            from: line.from,
            to: removeEnd,
            insert: ''
          },
          selection: { anchor: line.from },
          scrollIntoView: true
        });
        return true;
      }
    }

    const insertText = `\n${indent}${prefix} `;
    const cursorPos = head + insertText.length;

    view.dispatch({
      changes: {
        from: head,
        to: head,
        insert: insertText
      },
      selection: { anchor: cursorPos },
      scrollIntoView: true
    });
    return true;
  }

  const bulletMatch = /([ \t]*)(-)(\s)(.*)$/.exec(line.text);
  if (bulletMatch) {
    const content = bulletMatch[4];
    // Убеждаемся, что это не todo item
    if (!content.match(/^\[[ x]\]/)) {
      const indent = bulletMatch[1];
      const prefix = bulletMatch[2];
      const space = bulletMatch[3];

      if (content.trim() === '') {
        if (indent.length > 0) {
          return indentLess(view);
        } else {
          const removeEnd = line.from + bulletMatch[0].length;
          view.dispatch({
            changes: {
              from: line.from,
              to: removeEnd,
              insert: ''
            },
            selection: { anchor: line.from },
            scrollIntoView: true,
          });
          return true;
        }
      }

      const insertText = `\n${indent}${prefix}${space}`;
      const cursorPos = head + insertText.length;

      view.dispatch({
        changes: {
          from: head,
          to: head,
          insert: insertText
        },
        selection: { anchor: cursorPos },
        scrollIntoView: true
      });
      return true;
    }
  }

  return insertNewlineAndIndent(view);
}

function handleTab(view: EditorView): boolean {
  return indentMore(view);
}

function handleShiftTab(view: EditorView): boolean {
  return indentLess(view);
}

const listKeymap = keymap.of([
  { key: 'Enter', run: handleEnter },
  { key: 'Tab', run: handleTab },
  { key: 'Shift-Tab', run: handleShiftTab },
]);

const listTheme = EditorView.baseTheme({
  '.cm-bullet-point': {
    userSelect: 'none',
    pointerEvents: 'none'
  }
});

export const combinedListPlugin: Extension = [
  listExtension,
  listKeymap,
  listTheme
];
