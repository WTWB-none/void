<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { closePopup, get_themes_to_download, Theme } from '@/lib/logic/settings';
import Button from '../button/Button.vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import SettingsButton from './SettingsButton.vue';
let activeTab = ref('themes')
let show = defineModel();
let downloaded_themes = ref<string[]>([]);
let artifacts = ref<Theme[]>([]);

function download(key: string) {
  invoke('clone_theme', { key: key }).then(() => { console.log("Ok") }, (err) => { console.warn(err) });
}

listen('theme_downloaded', (event) => {
  const key = event.payload as string;
  downloaded_themes.value.push(key);
});

function changeType(selected: string) {
  activeTab.value = selected;
}

onMounted(() => {
  get_themes_to_download().then((res: Theme[]) => { artifacts.value = res; console.log(res); }, () => { console.warn("faild to fetch themes") });
})

</script>
<template>
  <div class="marketplace-backdrop" @click="(event) => (show = closePopup(event))">
    <div
      class="flex flex-col justify-evenly items-center gap-[1em] w-[80%] bg-[var(--card)] rounded-[var(--radius)] border-[1px solid var(--border)]">
      <h1 class="text-accent text-4xl">{{ $t('common.download') }}</h1>
      <div class="download-type">
        <SettingsButton :class="activeTab == 'themes' ? 'active' : 'inactive'" name="Темы"
          @click="changeType('themes')" />
        <SettingsButton :class="activeTab == 'plugins' ? 'active' : 'inactive'" name="Плагины"
          @click="changeType('plugins')" />
      </div>
      <div class="popup-marketplace">
        <template v-if="artifacts" v-for="object in artifacts">
          <div class="card" v-if="downloaded_themes.lastIndexOf(object.theme_name) == -1">
            <h1 class="text-xl text-accent ">{{ object.theme_name }}</h1>
            <img class="w-[80%] h-[40%] border-card-border" src="https://shadow.png">
            <p class="text-xs">{{ object.theme_author }}</p>
            <Button class="w-[80%]" @click="download(object.theme_name)">Скачать</Button>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.marketplace-backdrop {
  width: 100%;
  height: 100vh;
  position: absolute;
  top: 0;
  left: 0;
  z-index: 1000;
  display: flex;
  padding: 5em;
  align-items: center;
  justify-content: center;
  background: radial-gradient(circle at center, rgba(255, 255, 255, 1), 10%, rgba(0, 0, 0, 0.8));
}

.popup-marketplace {
  width: 100%;
  height: 80vh;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(20em, 1fr));
  grid-auto-rows: 22em;
  gap: 1rem;
  overflow-y: scroll;
}

.card {
  margin: 1rem;
  width: 18em;
  height: 22em;
  background-color: var(--card-nested);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-evenly;
}

h1 {
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.6);

}

.download-type {
  width: 100%;
  display: flex;
  justify-content: center;
  gap: 1.5em;
}

:deep(.active) {
  background-color: var(--destructive);
}

:deep(.inactive) {
  background-color: var(--accent);
}
</style>
