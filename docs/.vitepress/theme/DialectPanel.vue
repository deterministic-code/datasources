<script setup>
import { dialect } from "./dialectState.js";

defineProps({
  name: { type: String, required: true },
});
</script>

<template>
  <div v-show="dialect === name" class="dialect-panel">
    <slot />
  </div>
</template>
