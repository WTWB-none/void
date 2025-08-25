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
<script setup>
import SidePanel from "@/components/ui/side-panel/SidePanel.vue";
import SidebarProvider from "@/components/ui/sidebar/SidebarProvider.vue";
import SidebarTrigger from "@/components/ui/sidebar/SidebarTrigger.vue";
import {
  ChevronLeft,
  ChevronRight,
} from "lucide-vue-next";
import { onMounted, ref, watch } from "vue";
import { checkShowable } from "@/lib/logic/utils";
import { useSidebarStore } from "@/lib/logic/sidebarstore";
const sidebar_store = useSidebarStore();
let showTrigger = ref(false);
let showPanel = ref(sidebar_store.current);
watch(() => sidebar_store.current, () => {
  showPanel.value = sidebar_store.current;
})
onMounted(() => {
  showTrigger.value = checkShowable();
  window.addEventListener("resize", () => {
    showTrigger.value = checkShowable();
  });
});
</script>
<template>
  <div class="app-container">
    <SidebarProvider :defaultOpen="showPanel != 'collapsed'" :class="showTrigger ? 'max-w-[3em]' : 'max-w-[0px]'">
      <SidePanel />
      <SidebarTrigger class="sidepanel-trigger top-1" v-if="showTrigger"
        @click="() => { sidebar_store.toggle(); showPanel = sidebar_store.current; }" />
      <div class="text-sidebar-primary fixed right-[1%] top-1 flex gap-1">
        <ChevronLeft v-if="showTrigger" @click="$router.back()" />
        <ChevronRight v-if="showTrigger" @click="$router.forward()" />
      </div>
    </SidebarProvider>
    <Transition name="slide-fade">
      <div :class="showPanel == 'expanded' && checkShowable() ? 'content-view-with-sidebar' : 'content-view'">
        <Transition name="fade" duration="100" mode="out-in">
          <RouterView />
        </Transition>
      </div>
    </Transition>
  </div>
</template>

<style>
.app-container {
  width: 100%;
  display: flex;
  gap: 1rem;
}

.sidepanel-trigger {
  z-index: 100;
  position: fixed;
  left: 5%;
}

.content-view {
  margin-top: 3rem;
  margin-right: 1rem;
  width: 100%;
  height: 93vh;
  border: 1px solid var(--border);
  border-radius: 15.5px;
  background-color: var(--card);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15), inset 0 0 1px rgba(0, 0, 0, 0.05);
  transition: margin 0.1s ease-in-out, width 0.1s ease-in-out;
}

.content-view-with-sidebar {
  margin-top: 3rem;
  margin-right: 1rem;
  margin-left: 12rem;
  width: 90%;
  height: 93vh;
  border: 1px solid var(--border);
  border-radius: 15.5px;
  background-color: var(--card);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15), inset 0 0 1px rgba(0, 0, 0, 0.05);
  transition: margin 0.1s ease-in-out, width 0.1s ease-in-out;
}
</style>
