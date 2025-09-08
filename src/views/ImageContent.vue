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
<script setup lang="ts">
import { get_file_content, getFullPath } from '@/lib/logic/utils';
import { onMounted, ref, watch } from 'vue';
let props = defineProps({
  url: String
});

let file = ref('');

onMounted(async () => {
  if (!props.url) { return }
  let file_path = decodeURIComponent(atob(props.url));
  let path = await getFullPath(file_path);
  file.value = await get_file_content(path);
});
watch(() => props.url, async () => {
  if (!props.url) { return }
  let file_path = decodeURIComponent(atob(props.url));
  let path = await getFullPath(file_path);
  file.value = await get_file_content(path);
});
</script>
<template>
  <div class="h-[100%] flex items-center justify-center overflow-hidden" :key="$route.fullPath">
    <img class="w-max p-[3em]" :src="file" />
  </div>
</template>
