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

    input.addEventListener('input', () => this.api.setName(input.value));
    input.addEventListener('keydown', (e) => e.stopPropagation());
    input.addEventListener('keyup', (e) => e.stopPropagation());
    input.addEventListener('keypress', (e) => e.stopPropagation());
    input.addEventListener('mousedown', (e) => e.stopPropagation());
    input.addEventListener('click', (e) => e.stopPropagation());

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
