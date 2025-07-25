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
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import BentoGrid from "@/components/ui/bento-grid/BentoGrid.vue";
import BentoGridItem from "@/components/ui/bento-grid/BentoGridItem.vue";
import * as Vue from 'vue';
let username = ref("");
let component = ref();
const items = [
  {
    title: "The Dawn of Innovation",
    description: "Explore the birth of groundbreaking ideas and inventions.",
  },
  {
    title: "The Digital Revolution",
    description: "Dive into the transformative power of technology.",
  },
  {
    title: "The Art of Design",
    description: "Discover the beauty of thoughtful and experience design.",
  },
  {
    title: "The Power of Communication",
    description: "Understand the impact of effective communication in our lives.",
  },
  {
    title: "The Pursuit of Knowledge",
    description: "Join the quest for understanding and enlightenment.",
  },
  {
    title: "The Joy of Creation",
    description: "Experience the thrill of bringing ideas to life.",
  },
  {
    title: "The Spirit of Adventure",
    description: "Embark on exciting journeys and thrilling discoveries.",
  },
];
onMounted(async () => {
  username.value = await invoke("get_env", { ename: "name" });
  console.log(username.value);
  const code = await invoke('read_plugin', { name: 'test_plugin' })

  const blob = new Blob([code], { type: 'application/javascript' })
  const url = URL.createObjectURL(blob)

  const module = await import(/* @vite-ignore */ url)
  component.value = module.default


});
</script>
<template>
  <h1 class="text-4xl text-accent text-center pt-[1%]">
    {{ $t('home.welcome') }}, {{ username }}
  </h1>
  <div class="plugin-grid">
    <BentoGrid class="mx-auto max-w-4xl">
      <BentoGridItem v-for="(item, index) in items" :key="index"
        :class="index === 3 || index === 6 ? 'md:col-span-2 bg-[var(--card-nested)] nested' : 'bg-[var(--card-nested)] nested'">
        <template #header>
          <div class="flex size-full space-x-4">
            <div class="flex size-full flex-1 rounded-md bg-zinc-800"></div>
          </div>
        </template>

        <template #title>
          <strong>{{ item.title }}</strong>
        </template>

        <template #icon> </template>

        <template #description>
          <p>{{ item.description }}</p>
        </template>
      </BentoGridItem>
    </BentoGrid>
  </div>
</template>
<style scoped>
h1 {
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.6);
}

.plugin-grid {
  max-height: 80vh;
  overflow: scroll;
  margin-top: 2%;
}

.nested {
  box-shadow: inset 0 0 20px rgba(90, 50, 30, 0.2);
}
</style>
