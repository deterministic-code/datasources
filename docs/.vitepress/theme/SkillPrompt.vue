<script setup>
import { ref } from "vue";

defineProps({
  text: { type: String, required: true },
});

const input = ref(null);
const copied = ref(false);

const selectAll = () => input.value?.select();

const copy = async (text) => {
  await navigator.clipboard.writeText(text);
  copied.value = true;
  window.setTimeout(() => {
    copied.value = false;
  }, 1500);
};
</script>

<template>
  <div class="skill-prompt">
    <label class="skill-prompt-label" for="skill-prompt-input">Prompt</label>
    <div class="skill-prompt-row">
      <input
        id="skill-prompt-input"
        ref="input"
        class="skill-prompt-input"
        type="text"
        :value="text"
        readonly
        @focus="selectAll"
        @click="selectAll"
      />
      <button type="button" class="skill-prompt-copy" @click="copy(text)">
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

.skill-prompt-row {
  display: flex;
  gap: 0.5rem;
}

.skill-prompt-input {
  flex: 1;
  min-width: 0;
  height: 36px;
  padding: 0 10px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 6px;
  background: var(--vp-c-bg-alt);
  color: var(--vp-c-text-1);
  font: inherit;
}

.skill-prompt-copy {
  height: 36px;
  padding: 0 12px;
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
