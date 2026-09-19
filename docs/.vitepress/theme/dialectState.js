import { ref } from "vue";

export const DIALECTS = ["typescript", "csharp", "rust"];
export const DIALECT_LABELS = { typescript: "TypeScript", csharp: "C#", rust: "Rust" };
export const KEY = "datasources-dialect";
export const dialect = ref("typescript");

export const loadDialect = () => {
  if (typeof localStorage === "undefined") return;
  const stored = localStorage.getItem(KEY);
  if (stored && DIALECTS.includes(stored)) dialect.value = stored;
};

export const setDialect = (id) => {
  dialect.value = id;
  if (typeof localStorage !== "undefined") localStorage.setItem(KEY, id);
};
