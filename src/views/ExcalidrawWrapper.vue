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
import { ref, onMounted, watch } from 'vue';
import { applyReactInVue } from 'veaury';
import { appDataDir, join } from '@tauri-apps/api/path';
import { restore, serializeAsJSON } from '@excalidraw/excalidraw';
import { AppState, BinaryFiles } from '@excalidraw/excalidraw/types';
import { read_canvas, write_canvas } from '@/lib/logic/utils';
import { useExplorerStore } from '@/lib/logic/explorerstore';
import { ExcalidrawElement } from '@excalidraw/excalidraw/element/types';


let props = defineProps({
  url: String
});
interface ExcalidrawData {
  elements: ExcalidrawElement[];
  appState: Partial<AppState>;
  files?: BinaryFiles;
}

const ExcalidrawReact = ref<any>(null);
const drawingData = ref<ExcalidrawData>({
  elements: [],
  appState: {
    viewBackgroundColor: 'transparent',
    width: 0,
    height: 0,
    offsetTop: 0,
    offsetLeft: 0,
  }
});
const filePath = ref<string>('');
const file_path = ref<string>('');
const waiting = ref<boolean>(false);

const initializeApp = async () => {
  try {
    filePath.value = await join(await appDataDir(), 'drawing.json');
    const module = await import('@excalidraw/excalidraw');
    ExcalidrawReact.value = applyReactInVue(module.Excalidraw);
  } catch (error) {
    console.error('failed to start excalidraw');
  }
};

async function createExcalidrawInstance() {
  if (props.url != undefined) {
    let path = decodeURIComponent(atob(props.url));
    let content = await read_canvas(decodeURIComponent(atob(props.url)));
    if (content == '') {
      await initializeApp();
      file_path.value = useExplorerStore().current + '/' + path.split('/')[path.split('/').length - 1];
      return;
    }
    let obj = JSON.parse(content);
    let restored = restore(obj, null, null);
    drawingData.value = {
      elements: restored.elements,
      appState: {
        ...structuredClone(restored.appState),
        width: 0,
        height: 0,
        offsetTop: 0,
        offsetLeft: 0,
      },
      files: restored.files
    };
    file_path.value = useExplorerStore().current + '/' + path.split('/')[path.split('/').length - 1];
  }
  await initializeApp();
}

onMounted(async () => {
  await createExcalidrawInstance();
});

watch(() => props.url, async () => {
  createExcalidrawInstance();
})


const handleChange = (elements: ExcalidrawElement[], appState: AppState, files?: BinaryFiles) => {
  drawingData.value = { elements, appState, files };
  saveDrawing();
};

const saveDrawing = async () => {
  if (!filePath.value) return;
  let data = serializeAsJSON(drawingData.value.elements, drawingData.value.appState, drawingData.value.files ?? {}, "local");
  if (file_path.value == '' && !waiting.value) {
    waiting.value = true;
    file_path.value = await write_canvas(data);
  }
  else if (!waiting.value) {
    waiting.value = true;
    await write_canvas(data, file_path.value);
  }
  waiting.value = false;

};
</script>

<template>
  <div class="excalidraw-app w-full h-full overflow-hidden rounded-[15px]">
    <div class="canvas-container rounded-[15px] overflow-hidden">
      <component :is="ExcalidrawReact" :initialData="drawingData" @change="handleChange" :UIOptions="{
        canvasActions: {
          loadScene: false,
          saveScene: false,
          export: false,
          changeViewBackgroundColor: false,
          toggleTheme: false,
          saveAsImage: false,
        },
        libraryMenu: false,
      }" />
    </div>
  </div>
</template>

<style scoped>
.excalidraw-app {
  display: flex;
  flex-direction: column;
  background: transparent;
}

.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  color: #ff4444;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 20px;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

.canvas-container {
  flex: 1;
  position: relative;
}

:deep(.excalidraw) {
  --zIndex-popup: 1000;
}

:deep(.App-menu_top .HorizontalMenu) {
  padding-top: env(safe-area-inset-top);
}

:deep(.sidebar-trigger) {
  display: none !important;
}

:deep(.dropdown-menu-button) {
  display: none !important;
}
</style>
