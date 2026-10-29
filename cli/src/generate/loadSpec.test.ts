import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "../io.ts";
import { defaultDatasourceYaml, loadSpec } from "./loadSpec.ts";

const mem = (entries: string[], files: Record<string, string>): Fs => ({
  readFile: async (path) => {
    const text = files[path];
    if (text === undefined) throw new Error("ENOENT");
    return text;
  },
  writeFile: async () => {},
  readdir: async () => entries,
});

describe("loadSpec", () => {
  it("passes version diffs into generator input", async () => {
    const dir = "app";
    const path = "version-20261016T000000Z-001/diff.yaml";
    const loaded = await loadSpec(
      mem(["types.yaml", "version-20261016T000000Z-001"], {
        [join(dir, "types.yaml")]: "types: []\n",
        [join(dir, path)]: "types.yaml:\n  added: true\n",
      }),
      dir,
    );
    expect(loaded.files[path]).toMatch(/added: true/);
    expect(loaded.settings["version.diffs"]).toBe(path);
  });

  it("loads engine datasource files without copying sqlite onto datasource.yaml", async () => {
    const dir = "app";
    const sqlite = "version: 1.0.0\ntypes: []\n";
    const mysql = "version: 1.0.0\ntypes:\n  - base:\n      fields: []\n";
    const loaded = await loadSpec(
      mem(["types.yaml", "datasource.sqlite.yaml", "datasource.mysql.yaml"], {
        [join(dir, "types.yaml")]: "types: []\n",
        [join(dir, "datasource.sqlite.yaml")]: sqlite,
        [join(dir, "datasource.mysql.yaml")]: mysql,
      }),
      dir,
    );
    expect(loaded.files["datasource.sqlite.yaml"]).toBe(sqlite);
    expect(loaded.files["datasource.mysql.yaml"]).toBe(mysql);
    expect(loaded.files["datasource.yaml"]).toBe(defaultDatasourceYaml);
  });

  it("skips a version directory without a diff", async () => {
    const dir = "app";
    const loaded = await loadSpec(
      mem(["types.yaml", "version-20261016T000000Z-001"], {
        [join(dir, "types.yaml")]: "types: []\n",
      }),
      dir,
    );
    expect(loaded.settings["version.diffs"]).toBeUndefined();
  });
});
