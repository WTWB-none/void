<script setup lang="ts">
import { defineProps } from 'vue';
let props = defineProps({
  value: String,
});
</script>

<template>
  <p class="pl-10 text-xs pt-1">{{ props.value }}</p>
</template>
