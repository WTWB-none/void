<script setup lang="ts">
import { Theme, changeWorkdir, update, getWorkdir, get_installed_themes_list, get_themes_to_download, delete_theme } from "@/lib/logic/settings";
import { onMounted, ref } from "vue";
import { useThemeStore } from "@/lib/logic/themestore";
import SettingsSelector from "../components/ui/settings/SettingsSelector.vue";
import SettingsButton from "@/components/ui/settings/SettingsButton.vue";
import SettingsHeader from "@/components/ui/settings/SettingsHeader.vue";
import SettingsField from "@/components/ui/settings/SettingsField.vue";
import SettingsComposition from "@/components/ui/settings/SettingsComposition.vue";
import SettingsPopup from "@/components/ui/settings/SettingsPopup.vue";
import { listen } from "@tauri-apps/api/event";
let workdir = ref("");
let showPopup = ref(false);
let theme = ref("");
let objects = ref<Theme[]>([]);
let listOfThemes = ref();

function getThemesMarketplace() {
  showPopup.value = !showPopup.value;
  get_themes_to_download().then((res: Theme[]) => { objects.value = res; console.log(res); }, () => { console.warn("faild to fetch themes") });
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
  workdir.value = await getWorkdir();
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
  <h1 class="text-4xl text-center text-accent mt-2">Настройки</h1>
  <SettingsHeader value="изменить рабочую директорию" />
  <SettingsComposition>
    <SettingsField :placeholder="workdir" />
    <SettingsButton @click="async () => { workdir = await changeWorkdir(); }" name="Изменить" />
  </SettingsComposition>
  <SettingsHeader value="изменить тему" />
  <SettingsComposition>
    <SettingsSelector selectorPlaceholder="Темы" :currentVal="theme" :valList="listOfThemes" v-model="theme" />
    <SettingsButton @click="getThemesMarketplace" name="Скачать темы" />
  </SettingsComposition>
  <SettingsHeader value="Установленные темы" />
  <div class="installed-themes-table">
    <SettingsComposition v-for="theme in listOfThemes">
      <SettingsField :placeholder="theme" />
      <SettingsButton v-if="theme != 'lotm'" name="Проверить наличие обновлений"
        @click="async () => { update(theme) }" />
      <SettingsButton v-if="theme != 'lotm'" name="Удалить" @click="async () => { await delete_theme(theme) }" />
    </SettingsComposition>
  </div>
  <SettingsPopup v-if="showPopup" :object_list="objects" v-model="showPopup" />
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
