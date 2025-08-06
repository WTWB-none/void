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
import { onMounted, ref } from 'vue';
import { update, get_installed_themes_list, delete_theme, set_theme } from "@/lib/logic/settings";
import SettingsSelector from "../components/ui/settings/SettingsSelector.vue";
import SettingsHeader from '@/components/ui/settings/SettingsHeader.vue';
import SettingsButton from '@/components/ui/settings/SettingsButton.vue';
import SettingsComposition from '@/components/ui/settings/SettingsComposition.vue';
import SettingsField from '@/components/ui/settings/SettingsField.vue';
import SettingsPopup from "@/components/ui/settings/SettingsPopup.vue";
import { listen } from '@tauri-apps/api/event';
import { useThemeStore } from '@/lib/logic/themestore';
import SettingsSeparator from '@/components/ui/settings/SettingsSeparator.vue';
import { useI18n } from 'vue-i18n';
let { t } = useI18n();
let showPopup = ref(false);
let listOfThemes = ref();
let theme = ref("");

function getThemesMarketplace() {
  showPopup.value = !showPopup.value;
};

listen("theme_downloaded", async () => {
  await updateThemesList();
})

listen("theme_changed", async () => {
  await updateThemesList();
});

async function updateThemesList() {
  let themesArr = [];
  themesArr = await get_installed_themes_list();
  themesArr.push('lotm');
  listOfThemes.value = themesArr;
}

onMounted(async () => {
  let themeStore = useThemeStore();
  theme.value = themeStore.current;
  if (theme.value === '' && theme.value != null) {
    theme.value = 'lotm';
  }
  let themesArr = [];
  themesArr = await get_installed_themes_list();
  themesArr.push('lotm');
  listOfThemes.value = themesArr;
});
</script>
<template>
  <div class="w-full h-full flex flex-col gap-2 items-center overflow-auto">
    <h1 class="text-4xl text-[var(--destructive)] text-shadow-2xs">{{ $t('plugins.pluginHeader') }}</h1>
    <p> {{ $t('plugins.collect') }} void!</p>
  </div>
  <SettingsHeader :value="t('settingsHeaders.installPlugins')" />
  <SettingsComposition>
    <h2> {{ $t('plugins.moreFreedom') }}</h2>
    <SettingsButton :name="t('settingsButtons.installPlugins')" @click="async () => { getThemesMarketplace() }" />
  </SettingsComposition>
  <SettingsHeader :value="t('settingsHeaders.selectTheme')" />
  <SettingsComposition>
    <SettingsSelector selectorPlaceholder="Темы" :currentVal="theme" :valList="listOfThemes" v-model="theme"
      :exec-fn="set_theme" />
  </SettingsComposition>
  <SettingsHeader :value="t('settingsHeaders.installedThemes')" />
  <div class="installed-themes-table">
    <SettingsComposition v-for="theme in listOfThemes">
      <SettingsField :placeholder="theme" />
      <SettingsButton v-if="theme != 'lotm'" name="Проверить наличие обновлений"
        @click="async () => { update(theme) }" />
      <SettingsButton v-if="theme != 'lotm'" name="Удалить" @click="async () => { await delete_theme(theme) }" />
    </SettingsComposition>
  </div>
  <SettingsSeparator />
  <SettingsPopup v-if="showPopup" v-model="showPopup" />
</template>
<style scoped>
.installed-themes-table {
  margin-top: 1em;
  border: var(--border);
  margin-left: 3em;
  display: flex;
  flex-direction: column;
  gap: 1em;
}
</style>
