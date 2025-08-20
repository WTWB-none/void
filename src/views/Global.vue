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
  <h1 class="text-4xl text-center text-accent mt-2">{{ $t('common.general') }}</h1>
  <SettingsHeader :value="$t('settingsHeaders.changeWorkdir')" />
  <SettingsComposition>
    <SettingsField :placeholder="workdir" />
    <SettingsButton @click="async () => { workdir = await changeWorkdir(); }" :name="$t('settingsButtons.change')" />
  </SettingsComposition>
  <SettingsSeparator />
  <SettingsHeader :value="$t('settingsHeaders.editorSettings')" />
  <SettingsComposition>
    <div>{{ $t('settingsHeaders.changeLocale') }}</div>
    <SettingsSelector :selector-placeholder="$t('settingsSelector.language')" :val-list="listOfLocales"
      :current-val="locale.current" :exec-fn="set_locale" />
  </SettingsComposition>
  <SettingsComposition>
    <div>{{ $t("settings.editorMode") }}</div>
    <SettingsSelector :selector-placeholder="$t('settingsSelector.editorMode')" :val-list="['read', 'write']"
      :current-val="defaultEditorMode" :exec-fn="setEditorDefaultState" />
  </SettingsComposition>
  <SettingsSeparator />
  <SettingsHeader :value="$t('settingsHeaders.officialPlugins')" />
  <SettingsComposition v-for="plugin in listOfPlugins">
    <div>{{ plugin.plug.plugin_name }}</div>
    <Switch :model-value="plugin.enabled"
      @update:model-value="async () => { await changePluginState(plugin.plug.plugin_name, plugin.enabled); plugin.enabled = !plugin.enabled }" />
  </SettingsComposition>
</template>

<script setup lang="ts">
import { changeWorkdir, getWorkdir } from "@/lib/logic/settings";
import { onMounted, ref } from "vue";
import SettingsButton from "@/components/ui/settings/SettingsButton.vue";
import SettingsHeader from "@/components/ui/settings/SettingsHeader.vue";
import SettingsField from "@/components/ui/settings/SettingsField.vue";
import SettingsComposition from "@/components/ui/settings/SettingsComposition.vue";
import { useI18n } from 'vue-i18n';
import SettingsSelector from "@/components/ui/settings/SettingsSelector.vue";
import { useLocaleStore } from "@/lib/logic/locales";
import SettingsSeparator from "@/components/ui/settings/SettingsSeparator.vue";
import { Switch } from "@/components/ui/switch";
import { Plugin, changePluginState, get_plugins_list } from "@/lib/logic/extensions";
let workdir = ref("");
let listOfLocales = ref(useI18n().availableLocales);
let store = useI18n();
let locale = useLocaleStore();
let listOfPlugins = ref<{ plug: Plugin, enabled: boolean }[]>([]);
let defaultEditorMode = ref('');
onMounted(async () => {
  workdir.value = await getWorkdir();
  let plugins = await get_plugins_list("installed");
  plugins.filter((p) => { p.plugin_type == "official" });
  plugins.forEach((p) => {
    listOfPlugins.value.push({ plug: p, enabled: p.is_enabled === 'true' });
  })
  let ed = localStorage.getItem('mindbreaker:editorDefaults');
  if (ed != null) {
    defaultEditorMode.value = ed;
  }
  else {
    localStorage.setItem('mindbreaker:editorDefaults', 'write');
    defaultEditorMode.value = 'write';
  }
});
function set_locale(val: string) {
  locale.changeLocale(val);
  store.locale.value = val;
}

function setEditorDefaultState(val: string) {
  localStorage.setItem('mindbreaker:editorDefaults', val);
  defaultEditorMode.value = val;
}
</script>
