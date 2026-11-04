import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const src = (relative: string) => fileURLToPath(new URL(relative, import.meta.url));

export default defineConfig({
  resolve: {
    alias: {
      "@deterministic-code/datasources": src("../../datasources/src/index.ts"),
    },
  },
  test: {
    coverage: {
      provider: "v8",
      reporter: ["text", "json-summary"],
      include: ["src/**/*.ts"],
      exclude: ["src/index.ts", "src/**/*.test.ts", "src/connection-options.ts"],
      thresholds: {
        lines: 100,
        branches: 100,
        functions: 100,
        statements: 100,
      },
    },
  },
});
