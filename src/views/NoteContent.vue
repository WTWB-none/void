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
  <div class="w-full h-full pt-[3em] pl-[3em] pr-[3em]">
    <CodeMirror :extensions="extensions" v-model="content" ref="Editor" class="editor" />
  </div>
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

let props = defineProps({
  url: String
});
let content = ref('');
const extensions = shallowRef([CodeBlockPlugin, calloutExtension, quotePlugin, headingPlugin, inlinePlugin, pageBreaker, combinedListPlugin, hashtagField]);

watch(content, async () => {
  if (!props.url) { return }
  await write_note(decodeURIComponent(atob(props.url)), content.value);
})

onMounted(async () => {
  if (!props.url) { return }
  content.value = await get_note(decodeURIComponent(atob(props.url)));
})
</script>
<style></style>
