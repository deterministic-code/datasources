import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { ProviderNotInstalledError } from "@deterministic-code/datasources";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { generate } from "./generate-sql.ts";
import { ensureDialect, forgetDialects } from "./common/sql-dialect.ts";

const cwd = process.cwd();
const temps: string[] = [];

afterEach(() => {
  process.chdir(cwd);
  forgetDialects();
  for (const root of temps.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("generate-sql", () => {
  it("does not load sqlite from code-generators when the consumer has no package", async () => {
    const root = mkdtempSync(join(tmpdir(), "generate-sql-cwd-"));
    temps.push(root);
    writeFileSync(join(root, "package.json"), "{}");
    process.chdir(root);
    await assert.rejects(
      () =>
        generate(
          {
            reader: memoryReader({ "types.yaml": "types: []" }),
            settings: {},
          },
          ["sqlite"],
        ),
      ProviderNotInstalledError,
    );
  });

  it("loads sqlite from the consumer node_modules", async () => {
    const root = mkdtempSync(join(tmpdir(), "generate-sql-pkg-"));
    temps.push(root);
    writeFileSync(join(root, "package.json"), "{}");
    const dir = join(root, "node_modules/@deterministic-code/datasources-sqlite");
    mkdirSync(dir, { recursive: true });
    writeFileSync(
      join(dir, "package.json"),
      JSON.stringify({
        name: "@deterministic-code/datasources-sqlite",
        exports: { ".": { import: "./index.mjs" } },
      }),
    );
    writeFileSync(
      join(dir, "index.mjs"),
      `export class SqliteDriver {
  get statementRenderer() {
    return { withNaming() { return this; }, render() { return ""; } };
  }
}
`,
    );
    process.chdir(root);
    await expect(ensureDialect("sqlite")).resolves.toBe("sqlite");
  });

  it("throws when the dialect is not installed", async () => {
    await assert.rejects(
      () =>
        generate(
          {
            reader: memoryReader({ "types.yaml": "types: []" }),
            settings: {},
          },
          ["acme"],
        ),
      /Unknown SQL dialect/,
    );
  });
});
