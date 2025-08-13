
// obsSelection.ts
import { StateEffect } from '@codemirror/state';
import { EditorView, ViewPlugin, ViewUpdate } from '@codemirror/view';

const TAIL_MS = 140; // «хвост» после последнего selectionSet
export const obsForceRebuild = StateEffect.define<null>();

let selectingPointer = false;
let shiftDown = false;
let lastSelChangeAt = 0;

const activeViews = new Set<EditorView>();

export function obsIsSelectingNow(): boolean {
  return selectingPointer || shiftDown || (performance.now() - lastSelChangeAt) < TAIL_MS;
}

function scheduleRebuildAll(view: EditorView) {
  const win = view.dom.ownerDocument.defaultView!;
  win.requestAnimationFrame(() => {
    activeViews.forEach(v => v.dispatch({ effects: obsForceRebuild.of(null) }));
  });
}

export const ObsidianSelectionPlugin = ViewPlugin.fromClass(class {
  private onPointerDown = (e: PointerEvent) => { selectingPointer = e.buttons !== 0; };
  private onPointerUp = () => { selectingPointer = false; scheduleRebuildAll(this.view); };
  private onPointerCancel = () => { selectingPointer = false; scheduleRebuildAll(this.view); };
  private onDragEnd = () => { selectingPointer = false; scheduleRebuildAll(this.view); };
  private onBlur = () => { selectingPointer = false; scheduleRebuildAll(this.view); };
  private onKeyDown = (e: KeyboardEvent) => { if (e.key === 'Shift') shiftDown = true; };
  private onKeyUp = (e: KeyboardEvent) => { if (e.key === 'Shift') { shiftDown = false; scheduleRebuildAll(this.view); } };

  constructor(private view: EditorView) {
    activeViews.add(view);
    const doc = view.dom.ownerDocument;
    const win = doc.defaultView!;

    // capture=true, чтобы ловить событие даже вне редактора
    doc.addEventListener('pointerdown', this.onPointerDown, true);
    doc.addEventListener('pointerup', this.onPointerUp, true);
    doc.addEventListener('pointercancel', this.onPointerCancel, true);
    doc.addEventListener('dragend', this.onDragEnd, true);
    win.addEventListener('blur', this.onBlur, true);
    doc.addEventListener('keydown', this.onKeyDown, true);
    doc.addEventListener('keyup', this.onKeyUp, true);
  }

  update(u: ViewUpdate) {
    if (u.selectionSet) lastSelChangeAt = performance.now();
  }

  destroy() {
    activeViews.delete(this.view);
    const doc = this.view.dom.ownerDocument;
    const win = doc.defaultView!;
    doc.removeEventListener('pointerdown', this.onPointerDown, true);
    doc.removeEventListener('pointerup', this.onPointerUp, true);
    doc.removeEventListener('pointercancel', this.onPointerCancel, true);
    doc.removeEventListener('dragend', this.onDragEnd, true);
    win.removeEventListener('blur', this.onBlur, true);
    doc.removeEventListener('keydown', this.onKeyDown, true);
    doc.removeEventListener('keyup', this.onKeyUp, true);
  }
});
