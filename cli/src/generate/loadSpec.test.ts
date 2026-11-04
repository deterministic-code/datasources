import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "../io.ts";
import { defaultDatasourceYaml, loadSpec } from "./loadSpec.ts";
import { resolveIncludes } from "./resolveIncludes.ts";

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

  it("folds file includes and combine_options into types.yaml", async () => {
    const dir = "app";
    const loaded = await loadSpec(
      mem(["types.yaml"], {
        [join(dir, "types.yaml")]: `version: 1.0.0
includes:
  - file: ../users/types.yaml
    combine_options:
      source:
        remove_types: [base]
        add_fields:
          - type: user
            add_field:
              settings:
                type: settings_map{}
types:
  - extra:
      tags: [datasource_type]
      fields:
        - name:
            type: string
`,
        [join("users", "types.yaml")]: `version: 1.0.0
types:
  - base:
      fields:
        - id:
            type: integer
            is_id: true
  - user:
      tags: [datasource_type]
      inherits: base
      fields:
        - username:
            type: string
`,
      }),
      dir,
    );
    expect(loaded.files["types.yaml"]).not.toMatch(/file:/);
    expect(loaded.files["types.yaml"]).toMatch(/username:/);
    expect(loaded.files["types.yaml"]).toMatch(/settings:/);
    expect(loaded.files["types.yaml"]).toMatch(/extra:/);
    expect(loaded.files["types.yaml"]).not.toMatch(/\n  - base:\n/);
  });

  it("keeps a types filter include after folding file includes", async () => {
    const dir = "app";
    const loaded = await loadSpec(
      mem(["types.yaml"], {
        [join(dir, "types.yaml")]: `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
  - file: ../users/types.yaml
types: []
`,
        [join("users", "types.yaml")]: `version: 1.0.0
types:
  - user:
      tags: [datasource_type]
      fields:
        - username:
            type: string
`,
      }),
      dir,
    );
    expect(loaded.files["types.yaml"]).toMatch(/tag == "datasource_type"/);
    expect(loaded.files["types.yaml"]).toMatch(/username:/);
    expect(loaded.files["types.yaml"]).not.toMatch(/file:/);
  });

  it("leaves non-mapping yaml unchanged", async () => {
    expect(await resolveIncludes(mem([], {}), "types.yaml", "[]\n")).toBe("[]\n");
    expect(await resolveIncludes(mem([], {}), "types.yaml", "includes: {}\ntypes: []\n")).toBe(
      "includes: {}\ntypes: []\n",
    );
  });

  it("folds a child that only has a types filter include", async () => {
    const dir = "app";
    const loaded = await loadSpec(
      mem(["types.yaml"], {
        [join(dir, "types.yaml")]: `version: 1.0.0
includes:
  - file: ../users/types.yaml
types: []
`,
        [join("users", "types.yaml")]: `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - user:
      tags: [datasource_type]
      fields:
        - username:
            type: string
`,
      }),
      dir,
    );
    expect(loaded.files["types.yaml"]).toMatch(/username:/);
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
