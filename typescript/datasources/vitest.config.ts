import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    coverage: {
      provider: "v8",
      reporter: ["text", "json-summary"],
      include: ["src/**/*.ts"],
      exclude: [
        "src/**/*.test.ts",
        "src/ICommand.ts",
        "src/ICommandMiddleware.ts",
        "src/IConnection.ts",
        "src/IDatasource.ts",
        "src/IQuery.ts",
        "src/IReader.ts",
        "src/IRepository.ts",
        "src/IResponse.ts",
        "src/ITransaction.ts",
        "src/IWhere.ts",
        "src/DatasourceFactory.ts",
        "src/Parameter.ts",
        "src/Row.ts",
      ],
      thresholds: {
        lines: 100,
        branches: 100,
        functions: 100,
        statements: 100,
      },
    },
  },
});
