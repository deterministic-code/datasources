<script setup>
import { onMounted, onUnmounted, ref } from "vue";
import { SAMPLE_SIZES, SIZE_LABELS, SIZE_ORDER, TEXT_SIZES } from "./fontSize.js";
import { loadFontSize, sampleSize, setSampleSize, setTextSize, textSize } from "./fontSizeState.js";

const open = ref(false);
const root = ref(null);

const close = (event) => {
  if (!root.value?.contains(event.target)) open.value = false;
};

const onKey = (event) => {
  if (event.key === "Escape") open.value = false;
};

onMounted(() => {
  loadFontSize();
  document.addEventListener("pointerdown", close);
  document.addEventListener("keydown", onKey);
});

onUnmounted(() => {
  document.removeEventListener("pointerdown", close);
  document.removeEventListener("keydown", onKey);
});
</script>

<template>
  <div ref="root" class="font-size">
    <button
      type="button"
      class="font-size-toggle"
      title="Text size"
      aria-label="Text size"
      aria-haspopup="dialog"
      :aria-expanded="open"
      @click="open = !open"
    >
      Aa
    </button>
    <div v-if="open" class="font-size-panel" role="dialog" aria-label="Text size">
      <div class="font-size-row">
        <span class="font-size-label">Samples</span>
        <div class="font-size-options" role="radiogroup" aria-label="Sample text size">
          <button
            v-for="id in SIZE_ORDER"
            :key="id"
            type="button"
            role="radio"
            :aria-checked="sampleSize === id"
            :class="{ active: sampleSize === id }"
            @click="setSampleSize(id)"
          >
            <span class="font-size-name">{{ SIZE_LABELS[id] }}</span>
            <span class="font-size-px">{{ SAMPLE_SIZES[id] }}</span>
          </button>
        </div>
      </div>
      <div class="font-size-row">
        <span class="font-size-label">Text</span>
        <div class="font-size-options" role="radiogroup" aria-label="General text size">
          <button
            v-for="id in SIZE_ORDER"
            :key="id"
            type="button"
            role="radio"
            :aria-checked="textSize === id"
            :class="{ active: textSize === id }"
            @click="setTextSize(id)"
          >
            <span class="font-size-name">{{ SIZE_LABELS[id] }}</span>
            <span class="font-size-px">{{ TEXT_SIZES[id] }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.font-size {
  position: relative;
  display: flex;
  align-items: center;
  margin-left: 4px;
}

.font-size-toggle {
  width: 36px;
  height: 36px;
  border: 0;
  border-radius: 8px;
  background: transparent;
  color: var(--vp-c-text-2);
  font-size: 15px;
  font-weight: 650;
  letter-spacing: -0.04em;
  cursor: pointer;
}

.font-size-toggle:hover,
.font-size-toggle[aria-expanded="true"] {
  color: var(--vp-c-text-1);
  background: var(--vp-c-default-soft);
}

.font-size-panel {
  position: absolute;
  top: 100%;
  right: 0;
  z-index: 50;
  margin-top: 8px;
  padding: 12px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 12px;
  background: var(--vp-c-bg);
  box-shadow: var(--vp-shadow-3);
}

.font-size-row + .font-size-row {
  margin-top: 12px;
}

.font-size-label {
  display: block;
  margin-bottom: 6px;
  font-size: 12px;
  font-weight: 600;
  color: var(--vp-c-text-2);
}

.font-size-options {
  display: flex;
  gap: 4px;
}

.font-size-options button {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  min-width: 72px;
  padding: 6px 8px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: transparent;
  color: var(--vp-c-text-2);
  cursor: pointer;
}

.font-size-options button:hover {
  color: var(--vp-c-text-1);
}

.font-size-options button.active {
  color: var(--vp-c-text-1);
  border-color: var(--vp-c-brand-1);
  background: var(--vp-c-default-soft);
}

.font-size-name {
  font-size: 13px;
  font-weight: 600;
  line-height: 1.2;
}

.font-size-px {
  font-size: 11px;
  line-height: 1.2;
  color: var(--vp-c-text-3);
}
</style>
