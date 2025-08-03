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

class CalloutWidget extends WidgetType {
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

  toDOM(view: EditorView): HTMLElement {
    const box = document.createElement('div');
    box.className = `cm-callout callout-${this.tag.toLowerCase()}`;

    const headerEl = document.createElement('div');
    headerEl.className = 'callout-header';
    headerEl.textContent = this.header || this.tag;

    const bodyEl = document.createElement('div');
    bodyEl.className = 'callout-body';

    const nestedView = new NestedEditorView({
      doc: this.body,
      parent: bodyEl,
      extensions: [
        IsNestedEditor.of(true),
        calloutExtension,
        quotePlugin,
        inlinePlugin,
        combinedListPlugin,
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

    requestAnimationFrame(() => {
      nestedView.dispatch({
        effects: updateCalloutEffect.of(
          buildCalloutDecorations(nestedView.state, nestedView)
        )
      });
    });

    box.appendChild(headerEl);
    box.appendChild(bodyEl);
    if (!view.state.facet(IsNestedEditor)) {
      box.addEventListener('click', () => {
        view.dispatch(view.state.update({
          selection: EditorSelection.cursor(this.from),
          scrollIntoView: true,
        }))
        console.log(this.from);
        console.log(view.state.selection.main.head);
      })
    }
    return box;
  }

  ignoreEvent(event: Event): boolean {
    const target = event.target as HTMLElement;
    return !!(target.closest('.cm-callout') && !target.closest('input'));
  }
}

function parseCallouts(state: EditorState): {
  from: number;
  to: number;
  tag: string;
  header: string;
  body: string;
}[] {
  const lines = state.doc.toString().split('\n');
  const result = [];
  let i = 0;

  const headerRegex = /^>\s*\[!(?<tag>[A-Z]+)\](?<header>.*)/;
  const bodyRegex = /^>\s(?!\[)(?<body>.*)/;

  while (i < lines.length) {
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

  return result;
}

function buildCalloutDecorations(state: EditorState, view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const callouts = parseCallouts(state);
  const sel = state.selection.main;

  for (const { from, to, tag, header, body } of callouts) {
    const inside = sel.from >= from && sel.from <= to;
    if (!inside) {
      if (view.state.facet(IsNestedEditor)) {
        builder.add(
          from,
          to,
          Decoration.replace({
            widget: new CalloutWidget(tag, header, body, from, to, view),
            block: true,
            side: 1
          })
        );
      }
      else {
        builder.add(
          from,
          to,
          Decoration.replace({
            widget: new CalloutWidget(tag, header, body, from, to, view),
            block: true,
            side: 1
          })
        );
      }
    }
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
export const calloutExtension: Extension = [
  calloutDecorationField,
  calloutKeymap,
  EditorView.updateListener.of((update) => {
    if (update.docChanged || update.selectionSet || update.viewportChanged) {
      const view = update.view;
      const decorations = buildCalloutDecorations(view.state, view);
      view.dispatch({
        effects: updateCalloutEffect.of(decorations)
      });
    }
  })
];


const IsNestedEditor = Facet.define<boolean, boolean>({
  combine: values => values.length ? values[0] : false
});
