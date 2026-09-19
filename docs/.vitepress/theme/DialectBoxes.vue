<script setup>
import { onMounted, useSlots } from "vue";
import { DIALECTS, dialect, loadDialect } from "./dialectState.js";
import DialectSelector from "./DialectSelector.vue";

const slots = useSlots();
const langs = DIALECTS.filter((id) => slots[id]);

onMounted(loadDialect);
</script>

<template>
  <div class="dialect-boxes">
    <DialectSelector />
    <div v-for="id in langs" v-show="dialect === id" :key="id" class="dialect-boxes-panels">
      <slot :name="id" />
    </div>
  </div>
</template>

<style scoped>
.dialect-boxes :deep(.dialect-tabs) {
  margin-bottom: 8px;
}

.dialect-boxes-panels :deep(.dialect-box) {
  margin: 0 0 16px;
}

.dialect-boxes-panels :deep(.dialect-box:last-child) {
  margin-bottom: 0;
}

.dialect-boxes-panels :deep(.dialect-box-title) {
  margin: 0 0 8px;
  font-size: inherit;
  font-weight: inherit;
  color: inherit;
}

.dialect-boxes-panels :deep(div[class*="language-"]) {
  margin: 0;
}

.dialect-boxes-panels :deep(.dialect-box-title + *),
.dialect-boxes-panels :deep(> :first-child) {
  margin-top: 0;
}
</style>
