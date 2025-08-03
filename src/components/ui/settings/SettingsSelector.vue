<script setup lang="ts">
import Select from '../select/Select.vue';
import SelectItem from '../select/SelectItem.vue';
import SelectGroup from '../select/SelectGroup.vue';
import SelectLabel from '../select/SelectLabel.vue';
import SelectContent from '../select/SelectContent.vue';
import SelectValue from '../select/SelectValue.vue';
import SelectTrigger from '../select/SelectTrigger.vue';
import { defineProps, onMounted } from 'vue';
let props = defineProps({
  selectorPlaceholder: String,
  currentVal: String,
  valList: Array<string>,
  execFn: Function
});
onMounted(() => {
  if (props.execFn == undefined) {
    return null
  }
})
</script>
<template>
  <Select>
    <SelectTrigger>
      <SelectValue>{{ props.currentVal }}</SelectValue>
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectLabel>{{ props.selectorPlaceholder }}</SelectLabel>
        <SelectItem v-for="val in props.valList" :value="val"
          @select="() => { if (props.execFn != undefined) { props.execFn(val) } }"> {{ val
          }}
        </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
</template>
