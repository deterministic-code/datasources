import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { buildCustomMigrationFiles } from "./generate-custom-migrations.ts";

const temps: string[] = [];

afterEach(() => {
  for (const root of temps.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe("buildCustomMigrationFiles", () => {
  it("copies order 2 after the generated schema at that number", async () => {
    const root = mkdtempSync(join(tmpdir(), "custom-migrations-"));
    temps.push(root);
    const dir = join(root, "custom", "sqlite");
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "0002_seed_categories_up.sql"), "INSERT INTO categories VALUES (1);\n");
    writeFileSync(join(dir, "0002_seed_categories_down.sql"), "DELETE FROM categories WHERE id = 1;\n");
    const files = await buildCustomMigrationFiles(root, "sqlite");
    const up = files.find((file) => file.filename.endsWith("_up.sql"));
    expect(up?.filename).toBe("0002_up_custom_002_seed_categories_up.sql");
    const names = [
      "0001_initial_up.sql",
      "0001_up.sql",
      "0002_stored_procedures_up.sql",
      up!.filename,
      "0003_up.sql",
    ];
    expect([...names].sort()).toEqual([
      "0001_initial_up.sql",
      "0001_up.sql",
      "0002_stored_procedures_up.sql",
      "0002_up_custom_002_seed_categories_up.sql",
      "0003_up.sql",
    ]);
  });
});
