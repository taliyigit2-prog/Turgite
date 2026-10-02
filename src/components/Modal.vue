<script setup lang="ts">
import Icon from "./Icon.vue";

defineProps<{ title: string; wide?: boolean }>();
const emit = defineEmits<{ (e: "close"): void }>();
</script>

<template>
  <div class="modal-backdrop" @click.self="emit('close')">
    <div class="modal" :class="{ wide }">
      <div class="modal-head">
        <h3>{{ title }}</h3>
        <button class="icon-btn" @click="emit('close')">
          <Icon name="close" />
        </button>
      </div>
      <slot />
    </div>
  </div>
</template>
