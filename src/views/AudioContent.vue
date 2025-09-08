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
  <div class="audio-player select-none">
    <img :src="meta_image" />
    <audio ref="Player" :src="sound"></audio>
    <div class="controls">
      <h1 class="text-xl text-destructive">{{ meta_title }}</h1>
      <p class="text-xs">{{ meta_artist }}</p>
      <Progress v-if="Player && duration > 0" v-model="currentTime" :max="duration" class="w-[100%]" />
      <PlayCircle @click="togglePlay" v-if="!now_playing" class="w-[3em] h-[3em]" />
      <PauseCircle @click="togglePlay" v-else class="w-[3em] h-[3em]" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { get_audio_content, get_file_content, getFullPath } from '@/lib/logic/utils';
import { PlayCircle, PauseCircle } from 'lucide-vue-next';
import { Progress } from '@/components/ui/progress';
import { onMounted, onUnmounted, ref, nextTick, watch } from 'vue';

const props = defineProps<{ url: string }>();

const meta_image = ref('');
const meta_artist = ref('');
const meta_title = ref('');
const sound = ref('');
const Player = ref<HTMLAudioElement | null>(null);

const now_playing = ref(false);
const currentTime = ref(0);
const duration = ref(0);


function onTimeUpdate() {
  if (!Player.value) return;
  currentTime.value = Player.value.currentTime;
  duration.value = Player.value.duration;
}

function onLoadedMetadata() {
  if (!Player.value) return;
  duration.value = Player.value.duration;
}

function togglePlay() {
  if (!Player.value) return;
  if (Player.value.paused) {
    Player.value.play();
    now_playing.value = true;
  } else {
    Player.value.pause();
    now_playing.value = false;
  }
}

onMounted(async () => {
  if (!props.url) return;

  let audio_path = decodeURIComponent(atob(props.url));
  audio_path = await getFullPath(audio_path);
  sound.value = await get_file_content(audio_path);
  const response = await get_audio_content(audio_path);
  meta_image.value = response.picture_url;
  meta_artist.value = response.author;
  meta_title.value = response.title;

  await nextTick(); // Ждём монтирования <audio> (waiting for installation)

  if (Player.value) {
    Player.value.addEventListener('timeupdate', onTimeUpdate);
    Player.value.addEventListener('loadedmetadata', onLoadedMetadata);
  }
});

watch(() => props.url, async () => {
  if (!props.url) return;
  let audio_path = decodeURIComponent(atob(props.url));
  audio_path = await getFullPath(audio_path);
  sound.value = await get_file_content(audio_path);
  const response = await get_audio_content(audio_path);
  now_playing.value = false;
  meta_image.value = response.picture_url;
  meta_artist.value = response.author;
  meta_title.value = response.title;
  if (!Player.value) { return }
  currentTime.value = Player.value.currentTime;
  duration.value = Player.value.duration;

  await nextTick();

  if (Player.value) {
    Player.value.addEventListener('timeupdate', onTimeUpdate);
    Player.value.addEventListener('loadedmetadata', onLoadedMetadata);
  }
});

watch(() => Player.value?.pause(), () => {
  now_playing.value = false;
});

onUnmounted(() => {
  if (Player.value) {
    Player.value.removeEventListener('timeupdate', onTimeUpdate);
    Player.value.removeEventListener('loadedmetadata', onLoadedMetadata);
  }
});
</script>

<style scoped>
.audio-player {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
}

.controls {
  width: 50%;
  display: flex;
  gap: 0.5em;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>
