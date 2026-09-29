<script setup>
const props = defineProps({
  items: { type: String, required: true },
});

const boxes = JSON.parse(props.items);
const rows = (value) => Math.max(1, value.split("\n").length);
</script>

<template>
  <div class="example-outputs">
    <label v-for="box in boxes" :key="box.title" class="example-output">
      <span class="example-output-title">{{ box.title }}</span>
      <textarea
        class="example-output-value"
        :value="box.value"
        :rows="rows(box.value)"
        readonly
        spellcheck="false"
      />
    </label>
  </div>
</template>

<style scoped>
.example-outputs {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin: 12px 0 0;
}

.example-output {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.example-output-title {
  font-size: 0.8125em;
  font-weight: 600;
  color: var(--vp-c-text-2);
}

.example-output-value {
  box-sizing: border-box;
  width: 100%;
  resize: none;
  overflow: auto;
  font-family: var(--vp-font-family-mono);
  font-size: var(--docs-sample-size);
  line-height: 1.6;
  padding: 10px 12px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
  white-space: pre;
  field-sizing: content;
}
</style>
