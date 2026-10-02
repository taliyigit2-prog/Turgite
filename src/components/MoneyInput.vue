<script setup lang="ts">
import { ref, watch } from "vue";
import { minorToString, parseAmount } from "../lib/format";
import { currencySymbol } from "../lib/currencies";

const props = defineProps<{ modelValue: number; currency: string; placeholder?: string }>();
const emit = defineEmits<{ (e: "update:modelValue", v: number): void }>();

const text = ref(minorToString(props.modelValue, props.currency));

watch(
  () => props.modelValue,
  (v) => {
    if (parseAmount(text.value, props.currency) !== v) {
      text.value = minorToString(v, props.currency);
    }
  }
);

function onInput() {
  const v = parseAmount(text.value, props.currency);
  if (v !== null) emit("update:modelValue", v);
}
</script>

<template>
  <div style="display: flex; align-items: center; gap: 8px">
    <input
      class="input"
      type="text"
      inputmode="decimal"
      :value="text"
      :placeholder="placeholder || '0.00'"
      @input="
        text = ($event.target as HTMLInputElement).value;
        onInput();
      "
    />
    <span class="muted" style="font-weight: 600">{{ currencySymbol(currency) }}</span>
  </div>
</template>
