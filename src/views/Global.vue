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
import { changeWorkdir, getWorkdir } from "@/lib/logic/settings";
import { onMounted, ref } from "vue";
import SettingsButton from "@/components/ui/settings/SettingsButton.vue";
import SettingsHeader from "@/components/ui/settings/SettingsHeader.vue";
import SettingsField from "@/components/ui/settings/SettingsField.vue";
import SettingsComposition from "@/components/ui/settings/SettingsComposition.vue";
import { useI18n } from 'vue-i18n';
let { t } = useI18n();
let workdir = ref("");
onMounted(async () => {
  workdir.value = await getWorkdir();
});

</script>

<template>
  <h1 class="text-4xl text-center text-accent mt-2">{{ $t('common.general') }}</h1>
  <SettingsHeader :value="t('settingsHeaders.changeWorkdir')" />
  <SettingsComposition>
    <SettingsField :placeholder="workdir" />
    <SettingsButton @click="async () => { workdir = await changeWorkdir(); }" :name="t('settingsButtons.change')" />
  </SettingsComposition>
</template>
