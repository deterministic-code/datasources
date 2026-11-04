import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const src = (relative: string) => fileURLToPath(new URL(relative, import.meta.url));

export default defineConfig({
  resolve: {
    alias: {
      "@deterministic-code/datasources": src("../datasources/src/index.ts"),
    },
  },
  test: {
    include: ["src/**/*.test.ts"],
  },
});
