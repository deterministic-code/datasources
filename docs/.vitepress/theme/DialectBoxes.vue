<script setup>
import { ref, useSlots } from "vue";

const labels = { typescript: "TypeScript", csharp: "C#", rust: "Rust" };
const order = ["typescript", "csharp", "rust"];
const slots = useSlots();
const langs = order.filter((id) => slots[id]);
const current = ref(langs[0] ?? "typescript");
</script>

<template>
  <div class="dialect-boxes">
    <div class="dialect-boxes-tabs">
      <button
        v-for="id in langs"
        :key="id"
        type="button"
        :class="{ active: current === id }"
        @click="current = id"
      >
        {{ labels[id] ?? id }}
      </button>
    </div>
    <div v-for="id in langs" v-show="current === id" :key="id" class="dialect-boxes-panels">
      <slot :name="id" />
    </div>
  </div>
</template>

<style scoped>
.dialect-boxes-tabs {
  display: flex;
  gap: 0.25rem;
  margin-bottom: 8px;
}

.dialect-boxes-tabs button {
  padding: 0 12px;
  height: 36px;
  font-size: 14px;
  font-weight: 500;
  color: var(--vp-c-text-2);
  background: transparent;
  border: 0;
  border-bottom: 2px solid transparent;
  cursor: pointer;
}

.dialect-boxes-tabs button.active {
  color: var(--vp-c-text-1);
  border-bottom-color: var(--vp-c-brand-1);
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
</style>
