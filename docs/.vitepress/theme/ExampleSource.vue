<script setup>
import { onMounted, ref } from "vue";

defineProps({
  href: { type: String, required: true },
});

const link = ref(null);

onMounted(() => {
  const block = link.value?.parentElement?.querySelector('[class*="language-"]');
  if (block && link.value) block.appendChild(link.value);
});
</script>

<template>
  <a
    ref="link"
    :href="href"
    class="example-source-link"
    title="View test source"
    target="_blank"
    rel="noreferrer"
  >
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
      <path
        fill="currentColor"
        d="M3.75 1.5a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 0 0 .25-.25V6.5h-2.75A1.75 1.75 0 0 1 8.75 4.75V2H3.75Zm6.22.72 2.78 2.78h-2.03a.25.25 0 0 1-.25-.25V2.22ZM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v9.086A1.75 1.75 0 0 1 12.25 16h-8.5A1.75 1.75 0 0 1 2 14.25Zm4.75 9.5a.75.75 0 0 1 .75-.75h3.5a.75.75 0 0 1 0 1.5h-3.5a.75.75 0 0 1-.75-.75Zm0-3.5a.75.75 0 0 1 .75-.75h3.5a.75.75 0 0 1 0 1.5h-3.5a.75.75 0 0 1-.75-.75Z"
      />
    </svg>
  </a>
</template>
