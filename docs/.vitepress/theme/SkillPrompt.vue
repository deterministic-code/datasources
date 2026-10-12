<script setup>
import { computed, ref } from "vue";

const props = defineProps({
  text: { type: String, required: true },
});

const input = ref(null);
const copied = ref(false);
const rows = computed(() => {
  const lines = props.text.split("\n");
  const wrapped = lines.reduce((count, line) => count + Math.max(1, Math.ceil(line.length / 64)), 0);
  return Math.min(12, Math.max(3, wrapped));
});

const selectAll = () => {
  input.value?.focus();
  input.value?.select();
};

const copy = async () => {
  await navigator.clipboard.writeText(props.text);
  copied.value = true;
  window.setTimeout(() => {
    copied.value = false;
  }, 1500);
};
</script>

<template>
  <div class="skill-prompt">
    <label class="skill-prompt-label">Prompt</label>
    <div class="skill-prompt-box">
      <textarea
        ref="input"
        class="skill-prompt-input"
        :value="text"
        :rows="rows"
        readonly
        spellcheck="false"
        @focus="selectAll"
        @click="selectAll"
      />
      <button type="button" class="skill-prompt-copy" @click="copy">
        {{ copied ? "Copied" : "Copy" }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.skill-prompt {
  margin: 1rem 0;
}

.skill-prompt-label {
  display: block;
  margin-bottom: 0.35rem;
  font-weight: 600;
}

.skill-prompt-box {
  position: relative;
  min-width: 0;
}

.skill-prompt-input {
  box-sizing: border-box;
  display: block;
  width: 100%;
  min-width: 0;
  resize: vertical;
  padding: 8px 72px 8px 10px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 6px;
  background: var(--vp-c-bg-alt);
  color: var(--vp-c-text-1);
  font: inherit;
  line-height: 1.45;
  white-space: pre-wrap;
}

.skill-prompt-copy {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 1;
  height: 28px;
  padding: 0 10px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 6px;
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
  cursor: pointer;
}

.skill-prompt-copy:hover {
  border-color: var(--vp-c-brand-1);
  color: var(--vp-c-brand-1);
}
</style>
