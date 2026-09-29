<script setup>
import { dialect } from "./dialectState.js";

defineProps({
  csharp: { type: String, required: true },
  typescript: { type: String, required: true },
  rust: { type: String, required: true },
});
</script>

<template>
  <code>{{ { csharp, typescript, rust }[dialect] }}</code>
</template>
