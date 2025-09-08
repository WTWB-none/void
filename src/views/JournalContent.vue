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
  <div class="relative h-[89vh] w-full group select-none m-[1em]" tabindex="0">
    <div ref="scrollContainer"
      class="absolute h-[89vh] w-[calc(100%-2em)] rounded-[1em] inset-0 flex items-center justify-center overflow-auto"
      style="z-index: 10;">
      <VuePdfEmbed v-if="doc" :source="file" :page="currentPage" :style="pdfStyle" />
      <div v-else class="flex items-center justify-center w-full h-full text-white">
        {{ $t('journalContent.loadingMsg') }} PDF...
      </div>
    </div>

    <button class="absolute left-2 top-1/2 -translate-y-1/2
        bg-black/20 text-white text-3xl rounded-full w-14 h-14 flex items-center justify-center
        opacity-0 scale-90 invisible pointer-events-none
        group-hover:opacity-100 group-hover:scale-100 group-hover:visible group-hover:pointer-events-auto
        hover:bg-black/40 transition-all duration-200 z-30" @click="prevPage" :disabled="currentPage <= 1"
      :tabindex="-1" aria-label="Следующая страница">
      -</button>

    <button class="absolute right-2 top-1/2 -translate-y-1/2 mr-[1em]
        bg-black/20 text-white text-3xl rounded-full w-14 h-14 flex items-center justify-center
        opacity-0 scale-90 invisible pointer-events-none
        group-hover:opacity-100 group-hover:scale-100 group-hover:visible group-hover:pointer-events-auto
        hover:bg-black/40 transition-all duration-200 z-30" @click="nextPage" :disabled="currentPage >= pageCount"
      :tabindex="-1" aria-label="Next page">+</button>
    <div class="flex gap-2 items-center justify-center
        absolute left-1/2 bottom-8 -translate-x-1/2
        opacity-0 scale-90 invisible pointer-events-none
        group-hover:opacity-100 group-hover:scale-100 group-hover:visible group-hover:pointer-events-auto
        transition-all duration-200 z-30">
      <input
        class="border border-white/30 bg-black/20 text-white w-16 text-center rounded py-1 outline-none placeholder-white/60 transition-colors"
        type="number" :min="1" :max="pageCount" v-model.number="inputPage" @keydown.enter="goToInputPage"
        @blur="goToInputPage" :placeholder="currentPage.toString()" autocomplete="off" inputmode="numeric" />
      <span class="text-lg text-white select-none">/ {{ pageCount }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from "vue";
import VuePdfEmbed, { useVuePdfEmbed } from "vue-pdf-embed";
import { get_file_content, getFullPath } from "@/lib/logic/utils";

const props = defineProps<{ url: string }>();

const file = ref<string>("");
const { doc } = useVuePdfEmbed({ source: file });
const pageCount = computed(() => doc.value?.numPages || 1);

const currentPage = ref(1);
const inputPage = ref(1);

const scrollContainer = ref<HTMLDivElement | null>(null);

function scrollPage(offset: number) {
  const el = scrollContainer.value;
  if (el) el.scrollBy({ top: offset });
}

function goToInputPage() {
  if (inputPage.value < 1) inputPage.value = 1;
  if (inputPage.value > pageCount.value) inputPage.value = pageCount.value;
  currentPage.value = inputPage.value;
}

watch(currentPage, (val) => {
  if (val !== inputPage.value) inputPage.value = val;
});
watch(doc, (val) => {
  if (val) {
    currentPage.value = 1;
    inputPage.value = 1;
  }
});

async function loadPDF() {
  if (props.url) {
    const file_path = decodeURIComponent(atob(props.url));
    const path = await getFullPath(file_path);
    file.value = await get_file_content(path);
  }
}

function prevPage() {
  if (currentPage.value > 1) currentPage.value--;
}
function nextPage() {
  if (currentPage.value < pageCount.value) currentPage.value++;
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "ArrowLeft") {
    prevPage();
    e.preventDefault();
  }
  if (e.key === "ArrowRight") {
    nextPage();
    e.preventDefault();
  }
  if (e.key === "ArrowDown") {
    scrollPage(40);
    e.preventDefault();
  }
  if (e.key === "ArrowUp") {
    scrollPage(-40);
    e.preventDefault();
  }
}

onMounted(() => {
  loadPDF();
  window.addEventListener("keydown", handleKeydown);
});
onBeforeUnmount(() => {
  window.removeEventListener("keydown", handleKeydown);
});
watch(() => props.url, async () => {
  loadPDF();
});

const pdfStyle = {
  width: "100%",
  height: "100%",
  objectFit: "contain",
  maxWidth: "100%",
  maxHeight: "100%",
  display: "block",
  margin: "0 auto",
};
</script>

<style scoped>
input[type="number"]::-webkit-outer-spin-button,
input[type="number"]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

input[type="number"] {
  -moz-appearance: textfield;
  appearance: textfield;
}
</style>
