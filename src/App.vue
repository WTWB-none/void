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
import { RouterLink, useRouter } from 'vue-router';
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import router from './router/index'
import { RouterView } from 'vue-router';
import Loader from './views/Loader.vue';
import Alert from './components/ui/alert/Alert.vue';
import { listen } from '@tauri-apps/api/event';
import AlertTitle from './components/ui/alert/AlertTitle.vue';
import AlertDescription from './components/ui/alert/AlertDescription.vue';
import { Check, OctagonX } from 'lucide-vue-next';
import { useThemeStore } from './lib/logic/themestore';
import { set_theme } from './lib/logic/settings';
import { useI18n } from 'vue-i18n';
import { useLocaleStore } from './lib/logic/locales';
import { useFontStore } from './lib/logic/fonststore';
let show_anim = ref(true);
let error = ref('');
let notification = ref('');
let locale = useI18n();
let fontStore = useFontStore();
listen('error', (event) => {
  error.value = event.payload;
  console.log(error.value)
});

listen('notify', (event) => {
  notification.value = event.payload;
});

listen('theme_downloaded', (event) => {
  notification.value = 'Тема ' + event.payload + ' успешно установлена';
});

onMounted(async () => {
  let firstrun = await invoke('get_env', { ename: 'first_run' });
  let uname = await invoke('get_env', { ename: 'name' });
  let theme = localStorage.getItem('mindbreaker:theme');
  if (theme != 'lotm' && theme != null) {
    await set_theme(theme);
  }
  if (firstrun == "true") {
    router.push('/welcome');
  }
  else if (uname === "") {
    router.push('setup');
  }

  locale.locale.value = useLocaleStore().current;
  fontStore.loadFont();

  setTimeout(() => { show_anim.value = false }, 1500);

})
</script>
<template>
  <Loader v-if="show_anim" />
  <main v-else>
    <RouterView />
  </main>
  <Alert class="alert" v-if="error != ''" @click="error = ''">
    <OctagonX />
    <AlertTitle>{{ $t('notifications.error') }}</AlertTitle>
    <AlertDescription>{{ error }}</AlertDescription>
  </Alert>
  <Alert class="notification" v-if="notification != ''" @click="notification = ''">
    <Check />
    <AlertTitle>{{ $t('notifications.warning') }}</AlertTitle>
    <AlertDescription>{{ notification }}</AlertDescription>
  </Alert>
</template>
<style>
body {
  background-color: var(--background);
  width: 100%;
  min-height: 100vh;
  overflow-y: hidden;
  font-family: Spectral;
  color: var(--card-foreground)
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.alert {
  z-index: 1000;
  width: 40%;
  position: absolute;
  bottom: 10px;
  right: 10px;
  color: var(--destructive);
  border: 1px solid var(--destructive);
}

.notification {
  z-index: 1000;
  width: 40%;
  position: absolute;
  bottom: 10px;
  right: 10px;
  color: var(--primary);
  border: 1px solid var(--primary);
}
</style>
