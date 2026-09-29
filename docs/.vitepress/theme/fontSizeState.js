import { ref } from "vue";
import { SAMPLE_KEY, SAMPLE_SIZES, TEXT_KEY, TEXT_SIZES } from "./fontSize.js";

export const sampleSize = ref("medium");
export const textSize = ref("medium");

const stored = (sizes, key) => {
  const value = localStorage.getItem(key);
  return value in sizes ? value : "medium";
};

const paint = () => {
  const root = document.documentElement;
  root.style.setProperty("--docs-sample-size", SAMPLE_SIZES[sampleSize.value]);
  root.style.setProperty("--docs-text-size", TEXT_SIZES[textSize.value]);
};

export const loadFontSize = () => {
  if (typeof localStorage === "undefined") return;
  sampleSize.value = stored(SAMPLE_SIZES, SAMPLE_KEY);
  textSize.value = stored(TEXT_SIZES, TEXT_KEY);
  paint();
};

const save = (sizes, key, size, id) => {
  if (!(id in sizes)) return;
  size.value = id;
  localStorage.setItem(key, id);
  paint();
};

export const setSampleSize = (id) => save(SAMPLE_SIZES, SAMPLE_KEY, sampleSize, id);

export const setTextSize = (id) => save(TEXT_SIZES, TEXT_KEY, textSize, id);
