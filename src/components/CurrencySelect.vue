<script setup lang="ts">
import { CURRENCIES } from "../lib/currencies";

const props = defineProps<{ modelValue: string }>();
const emit = defineEmits<{ (e: "update:modelValue", v: string): void }>();
</script>

<template>
  <select
    class="select"
    :value="modelValue"
    @change="emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
  >
    <option v-for="c in CURRENCIES" :key="c.code" :value="c.code">
      {{ c.code }} — {{ c.name }}
    </option>
  </select>
</template>
