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
  ViewUpdate
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
import { keymap } from '@codemirror/view';

const updateCalloutEffect = StateEffect.define<DecorationSet>();

// Кеш для виджетов callout
const calloutWidgetCache = new Map<string, CalloutWidget>();

class CalloutWidget extends WidgetType {
  private nestedView?: NestedEditorView;
  private domElement?: HTMLElement;

  constructor(
    private readonly tag: string,
    private readonly header: string,
    private readonly body: string,
    private readonly from: number,
    private readonly to: number,
    private readonly outerView: EditorView
  ) {
    super();
  }

  // Создаем уникальный ключ для кеширования
  get cacheKey(): string {
    return `${this.tag}-${this.header}-${this.body}`;
  }

  eq(other: CalloutWidget): boolean {
    return this.cacheKey === other.cacheKey;
  }

  toDOM(view: EditorView): HTMLElement {
    // Проверяем кеш
    const cached = calloutWidgetCache.get(this.cacheKey);
    if (cached?.domElement) {
      return cached.domElement;
    }

    const box = document.createElement('div');
    const line = document.createElement('div');
    line.className = 'cm-line';
    box.className = `cm-callout callout-${this.tag.toLowerCase()}`;

    const headerEl = document.createElement('div');
    headerEl.className = 'callout-header';
    headerEl.textContent = this.header || this.tag;

    const bodyEl = document.createElement('div');
    bodyEl.className = 'callout-body';

    // Создаем nested view только если его еще нет
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
            if (update.docChanged) {
              const newText = update.state.doc.toString();
              this.outerView.dispatch({
                changes: {
                  from: this.from,
                  to: this.to,
                  insert:
                    `> [!${this.tag}] ${this.header}\n` +
                    newText
                      .split('\n')
                      .map(line => `> ${line}`)
                      .join('\n')
                }
              });
            }
          })
        ]
      });

      // Обновляем decorations для nested view асинхронно
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
        view.dispatch(view.state.update({
          selection: EditorSelection.cursor(this.to),
          scrollIntoView: true,
        }))
      });
      box.appendChild(editButton);
    }

    box.appendChild(headerEl);
    box.appendChild(bodyEl);
    box.style.overflow = 'hidden';

    if (!view.state.facet(IsNestedEditor)) {
      let estimatedHeight = (this.body.split('\n').length + 2 + this.body.split('\n').filter((a) => {
        return a.includes('> [!');
      }).length) * view.defaultLineHeight + (this.body.split('\n').filter((a) => {
        return a.includes('> [!');
      }).length * 75);
      if (this.body.split('\n').filter((el) => {
        return el.includes('> [!');
      }).length == 0) {
        estimatedHeight += 40;
      }
      box.style.setProperty('height', `${estimatedHeight}px`, 'important');
    }

    line.appendChild(box);
    this.domElement = line;

    // Сохраняем в кеш
    calloutWidgetCache.set(this.cacheKey, this);

    return line;
  }

  ignoreEvent(event: Event): boolean {
    const target = event.target as HTMLElement;
    return !!target.closest('.callout-edit');
  }

  destroy() {
    if (this.nestedView) {
      this.nestedView.destroy();
      this.nestedView = undefined;
    }
    calloutWidgetCache.delete(this.cacheKey);
    this.domElement = undefined;
  }
}

interface CalloutData {
  from: number;
  to: number;
  tag: string;
  header: string;
  body: string;
}

// Кеш для парсинга callouts
let lastDocVersion = 0;
let cachedCallouts: CalloutData[] = [];

function parseCallouts(state: EditorState): CalloutData[] {
  // Проверяем, изменился ли документ
  const currentVersion = state.doc.length; // Простая проверка
  if (currentVersion === lastDocVersion && cachedCallouts.length > 0) {
    return cachedCallouts;
  }

  const lines = state.doc.toString().split('\n');
  const result: CalloutData[] = [];
  let i = 0;

  const headerRegex = /^>\s*\[!(?<tag>[A-Z]+)\](?<header>.*)/;
  const bodyRegex = /^>\s(?!\[)(?<body>.*)/;

  while (i < lines.length) {
    console.log(
      'first char code:', lines[i].charCodeAt(1),
      'line text:', JSON.stringify(lines[i])
    );
    const headerMatch = lines[i].match(headerRegex);
    if (!headerMatch?.groups) {
      i++;
      continue;
    }

    const tag = headerMatch.groups.tag;
    const header = headerMatch.groups.header.trim();
    const bodyLines = [];
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

    result.push({
      from,
      to,
      tag,
      header,
      body: bodyLines.join('\n')
    });

    i = toLine + 1;
  }

  // Кешируем результат
  lastDocVersion = currentVersion;
  cachedCallouts = result;

  return result;
}

function buildCalloutDecorations(state: EditorState, view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const callouts = parseCallouts(state);
  const sel = state.selection.main;

  for (const { from, to, tag, header, body } of callouts) {
    const inside = sel.from >= from && sel.from <= to;
    if (!inside) {
      // Проверяем, есть ли уже виджет в кеше
      const cacheKey = `${from}-${to}-${tag}-${header}-${body}`;
      let widget = calloutWidgetCache.get(cacheKey);

      if (!widget) {
        widget = new CalloutWidget(tag, header, body, from, to, view);















      }

      builder.add(
        from,
        to,
        Decoration.replace({
          widget,
          side: 1
        })
      );
    }
  }

  return builder.finish();
}

const calloutDecorationField = StateField.define<DecorationSet>({
  create(state) {
    return Decoration.none;
  },
  update(deco, tr) {
    // Обрабатываем эффекты
    for (const e of tr.effects) {
      if (e.is(updateCalloutEffect)) return e.value;
    }

    // Если есть изменения в документе, но нет эффектов - мапим существующие decorations
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
        changes: {
          from: line.from,
          to: line.to,
          insert: reducedPrefix
        },
        selection: { anchor: line.from + reducedPrefix.length },
        scrollIntoView: true
      });
    } else {
      view.dispatch({
        changes: {
          from: line.from,
          to: line.to,
          insert: ''
        },
        selection: { anchor: line.from },
        scrollIntoView: true
      });
    }
    return true;
  }

  const insertText = `\n${cleanPrefix}`;
  view.dispatch({
    changes: {
      from: head,
      to: head,
      insert: insertText
    },
    selection: { anchor: head + insertText.length },
    scrollIntoView: true
  });

  return true;
}

const calloutKeymap = keymap.of([
  { key: 'Enter', run: handleEnterForCallout }
]);

// Дебаунсинг обновлений
let updateTimeout: number | undefined;

export const calloutExtension: Extension = [
  calloutDecorationField,
  calloutKeymap,
  EditorView.updateListener.of((update) => {
    if (update.docChanged || update.selectionSet || update.viewportChanged) {
      // Очищаем предыдущий таймаут
      if (updateTimeout !== undefined) {
        clearTimeout(updateTimeout);
      }

      // Для изменений в селекции - обновляем сразу
      if (update.selectionSet && !update.docChanged) {
        const view = update.view;
        const decorations = buildCalloutDecorations(view.state, view);
        view.dispatch({
          effects: updateCalloutEffect.of(decorations)
        });
        return;
      }

      // Для изменений документа - дебаунсим
      updateTimeout = window.setTimeout(() => {
        const view = update.view;
        const decorations = buildCalloutDecorations(view.state, view);
        view.dispatch({
          effects: updateCalloutEffect.of(decorations)
        });
        updateTimeout = undefined;
      }, 10); // Очень короткая задержка
    }
  })
];


export const IsNestedEditor = Facet.define<boolean, boolean>({
  combine: values => values.length ? values[0] : false
});

