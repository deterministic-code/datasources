<script setup>
import { onMounted } from "vue";
import { DIALECTS, DIALECT_LABELS, dialect, loadDialect, setDialect } from "./dialectState.js";

onMounted(loadDialect);
</script>

<template>
  <div class="dialect-tabs" role="tablist" aria-label="Language">
    <button
      v-for="id in DIALECTS"
      :key="id"
      type="button"
      role="tab"
      :aria-selected="dialect === id"
      :class="{ active: dialect === id }"
      @click="setDialect(id)"
    >
      {{ DIALECT_LABELS[id] }}
    </button>
  </div>
</template>
