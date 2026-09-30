<script setup>
const props = defineProps({
  items: { type: String, required: true },
});

const boxes = JSON.parse(props.items);
const rows = (value) => Math.min(12, Math.max(1, value.split("\n").length));
</script>

<template>
  <div class="example-schemas">
    <label v-for="box in boxes" :key="box.name" class="example-schema">
      <a class="example-schema-title" :href="box.href">{{ box.name }}</a>
      <textarea
        class="example-schema-sql"
        :value="box.sql"
        :rows="rows(box.sql)"
        readonly
        spellcheck="false"
      />
    </label>
  </div>
</template>

<style scoped>
.example-schemas {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin: 12px 0 0;
}

.example-schema {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.example-schema-title {
  font-size: 0.8125em;
  font-weight: 600;
}

.example-schema-sql {
  box-sizing: border-box;
  width: 100%;
  resize: vertical;
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
