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
    <CodeMirror :extensions="extensions" v-model="content" ref="Editor" class="editor" :onmousedown="enableSelection"
      :onmouseup="stopSelection" />
  </EditorProvider>
</template>
<script setup lang="ts">
import { onMounted, ref, shallowRef, watch } from 'vue';
import CodeMirror from 'vue-codemirror6';
import { headingPlugin } from '@/components/editor/headers/headers';
import { pageBreaker } from '@/components/editor/page-breaker/page-breaker';
import { inlinePlugin } from '@/components/editor/inline/inline';
import { quotePlugin } from '@/components/editor/quote/quote';
import { combinedListPlugin } from '@/components/editor/lists/lists';
import { calloutExtension } from '@/components/editor/callout/callout';
import { hashtagField } from '@/components/editor/tags/tags';
import { CodeBlockPlugin } from '@/components/editor/code-block/codeblock';
import { get_note, write_note } from '@/lib/logic/md-notes';
import { EditorView } from '@codemirror/view';
import { useSelectionStore } from '@/lib/logic/selectorStore';
import { decide_file_ext, get_env, rename } from '@/lib/logic/utils';
import { useExplorerStore } from '@/lib/logic/explorerstore';
import EditorProvider from '@/components/editor/provider/EditorProvider.vue';
import router from '@/router';

let props = defineProps({
  url: String
});
let selection = useSelectionStore();
let content = ref<string>('');
let filename = ref<string>('');
const extensions = shallowRef([EditorView.lineWrapping, CodeBlockPlugin, calloutExtension, quotePlugin, headingPlugin, inlinePlugin, pageBreaker, combinedListPlugin, hashtagField]);
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
    console.log(filename.value);
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
  console.log(decodeURIComponent(atob(props.url)));
  filename.value = decodeURIComponent(atob(props.url)).split('/')[decodeURIComponent(atob(props.url)).split('/').length - 1];
  content.value = '# ' + filename.value.replace('.md', '') + '\n';
  content.value += await get_note(decodeURIComponent(atob(props.url)));
})
</script>
<style></style>
