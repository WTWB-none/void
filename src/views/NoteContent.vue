<!--
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
-->
<template>
  <EditorProvider>
    <CodeMirror ref="cm" :extensions="extensions" v-model="content" class="editor" :onmousedown="enableSelection"
      :onmouseup="stopSelection" :disabled="editorDefaults" />
  </EditorProvider>
</template>
<script setup lang="ts">
import { onMounted, ref, shallowRef, watch } from 'vue';
import CodeMirror from 'vue-codemirror6';
import { get_note, write_note } from '@/lib/logic/md-notes';
import { EditorView } from '@codemirror/view';
import { useSelectionStore } from '@/lib/logic/selectorStore';
import { decide_file_ext, get_env, rename } from '@/lib/logic/utils';
import { useExplorerStore } from '@/lib/logic/explorerstore';
import EditorProvider from '@/components/editor/provider/EditorProvider.vue';
import router from '@/router';
import { get_official_plugin, get_plugins_list } from '@/lib/logic/extensions';
let cm = ref<InstanceType<typeof CodeMirror>>();
let props = defineProps({
  url: String
});
let editorDefaults = ref<boolean>(localStorage.getItem('mindbreaker:editorDefaults') == 'read');
let selection = useSelectionStore();
let content = ref<string>('');
let filename = ref<string>('');
const extensions = shallowRef([EditorView.lineWrapping])
function enableSelection() {
  selection.toggleTrue();
}
function stopSelection() {
  selection.toggleFalse();
}
watch(content, async () => {
  if (!props.url) { return }
  let new_filename = content.value.split('\n')[0].replace('# ', '').replace('\n', '');
  if (filename.value != new_filename && new_filename != '' && new_filename != '#') {
    let explorer = useExplorerStore();
    let workdir = await get_env('workdir');
    let elder_name = filename.value
    filename.value = new_filename;
    await rename(decodeURIComponent(atob(props.url)).replace(workdir, ''), filename.value + '.md');
    await decide_file_ext(decodeURIComponent(atob(props.url)).replace(workdir + explorer.current, '').replace(elder_name + '.md', filename.value + '.md'), router);
  }
  else {
    await write_note(decodeURIComponent(atob(props.url)), content.value.replace('# ' + filename.value.replace('.md', '') + '\n', ''));
  }
});

onMounted(async () => {
  if (!props.url) { return }
  filename.value = decodeURIComponent(atob(props.url)).split('/')[decodeURIComponent(atob(props.url)).split('/').length - 1];
  content.value = '# ' + filename.value.replace('.md', '') + '\n';
  content.value += await get_note(decodeURIComponent(atob(props.url)));
  let enabled_extensions = await get_plugins_list('installed');
  let filt = enabled_extensions.filter((v) => { if (v.plugin_type == 'official') { return v } });
  filt = filt.filter((v) => { if (v.is_enabled == 'true') { return v } });
  const orderMap: Record<string, number> = { callout: 0, quote: 1 };
  const order = (name: string) => (name in orderMap ? orderMap[name] : 2);

  filt.sort((a, b) => {
    const oa = order(a.plugin_name);
    const ob = order(b.plugin_name);
    if (oa !== ob) return oa - ob;
    return a.plugin_name.localeCompare(b.plugin_name);
  });

  let loaded = await Promise.all(filt.map((p) => get_official_plugin(p.plugin_name)));
  loaded = loaded.filter((p) => { if (p !== null) { return p } });
  extensions.value = [...extensions.value, loaded.flat()];
  requestAnimationFrame(() => {
    if (cm.value == undefined) return;
    cm.value.view.dispatch({ selection: { anchor: cm.value.view.state.selection.main.anchor }, })
  })
  document.addEventListener('keypress', (e) => {
    if (e.metaKey && e.key == 'e') {
      editorDefaults.value = !editorDefaults.value;
    }
  })
})
</script>
