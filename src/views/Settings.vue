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
import Card from "@/components/ui/profile/Card.vue";
import Global from "./Global.vue";
import SettingsSelector from "./SettingsSelector.vue";
import { onMounted, ref, watch } from "vue";
import type { Component } from "vue";
import { getUsername, getWorkdir } from "@/lib/logic/settings";
import { checkShowable, get_file_content } from "@/lib/logic/utils";
import { invoke } from "@tauri-apps/api/core";

const uname = ref("");
const pic = ref();
const showCard = ref(true);
const workdir = ref("");
const settings_type = ref("");
const settingsComponent = ref<Component | null>(null);
const componentModules = import.meta.glob("./*.vue");
const componentMap: Record<string, () => Promise<any>> = {};
for (const path in componentModules) {
  const name = path.split("/").pop()?.replace(".vue", "")!;
  componentMap[name] = componentModules[path];
}

async function get_settings(name: string): Promise<Component> {
  if (!name || name === "Global") return Global;
  const loader = componentMap[name];
  if (!loader) {
    console.warn(`Component '${name}' not found`);
    return Global;
  }
  const mod = await loader();
  return mod.default;
}

watch(settings_type, async () => {
  settingsComponent.value = await get_settings(settings_type.value);
});

onMounted(async () => {
  uname.value = await getUsername();
  workdir.value = await getWorkdir();
  showCard.value = checkShowable();
  let profile_pic = await invoke<string>('get_config_directory');
  profile_pic += '/profile.png';
  pic.value = await get_file_content(profile_pic);

  window.addEventListener("resize", () => {
    showCard.value = checkShowable();
  });
});
</script>

<template>
  <div class="settings-container">
    <Global v-if="settings_type === '' || settings_type === 'Global'" />
    <component v-else :is="settingsComponent" />
    <div class="user-3d fixed right-[5%] top-[20%]" v-if="showCard">
      <Card :uname="uname" :pic="pic" />
    </div>
    <div class="fixed right-[6%] top-[15%]">
      <SettingsSelector v-model="settings_type" @select="
        async () => {
          settingsComponent = await get_settings(settings_type);
        }
      " />
    </div>
  </div>
</template>

<style scoped>
h1 {
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.6);
}

.settings-container {
  max-height: 100%;
  overflow-y: scroll;
}
</style>
