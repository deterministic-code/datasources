import { join } from "node:path";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { content, patch } from "@deterministic-code/generators-common/generate-entry";
import { DeterministicParser } from "@deterministic-code/deterministic-specifications-typescript/parser";
import { describe, expect, it } from "vitest";
import type { Fs } from "../io.ts";
import { run, type Deps } from "../run.ts";
import type { Prompter } from "../setup.ts";
import { deterministic } from "./DeterministicProvider.ts";
import type { Dialect } from "./GenerateRequest.ts";
import { planRepositories } from "./RepositoryPlan.ts";
import { providerByName } from "./providers.ts";
import { flattenSettings, settingsFrom } from "./settingsFlat.ts";
import { writeEntries } from "./writeEntries.ts";

const typesYaml = `version: 1.0.0
types:
  - base:
      tags: [datasource_type]
      fields:
      - id:
          type: integer
          is_id: true
      - uuid:
          type: uuid
      - created:
          type: datetime
      - updated:
          type: datetime
      - version:
          type: binary
  - contact_source:
      tags: [datasource_type, readonly_lookup]
      inherits: base
      fields:
      - name:
          type: string
          size: 64
      - description:
          type: string
          size: unlimited
          is_nullable: true
  - contacts_base:
      tags: [datasource_type]
      inherits: base
      fields:
      - contact_source_id:
          references: contact_source.id
      - first_name:
          type: string
          size: 128
      - last_name:
          type: string
          size: 128
      - email:
          type: string
          size: 256
          is_nullable: true
      - notes:
          type: string
          size: unlimited
          is_nullable: true
  - addresses_base:
      tags: [datasource_type]
      inherits: base
      fields:
      - contact_id:
          references: contacts_base.id
      - line1:
          type: string
          size: 256
      - line2:
          type: string
          size: 256
          is_nullable: true
      - city:
          type: string
          size: 128
      - region:
          type: string
          size: 128
          is_nullable: true
      - postal_code:
          type: string
          size: 32
          is_nullable: true
      - country:
          type: string
          size: 64
          is_nullable: true
  - phones_base:
      tags: [datasource_type]
      inherits: base
      fields:
      - contact_id:
          references: contacts_base.id
      - number:
          type: string
          size: 32
      - label:
          type: string
          size: 32
          is_nullable: true
  - contact_groups_base:
      tags: [datasource_type]
      inherits: base
      fields:
      - name:
          type: string
          size: 128
      - description:
          type: string
          size: unlimited
          is_nullable: true
  - contact:
      tags: [view_type]
      inherits: contacts_base
      union: [contact_source]
      mapping:
        name: contact_source_name
      remove_fields: [contact_source.id, contact_source.uuid, contact_source.created, contact_source.updated, contact_source.version]
      fields:
      - addresses:
          type: address[]
          references: addresses_base.contact_id
      - phones:
          type: phone[]
          references: phones_base.contact_id
  - address:
      tags: [view_type]
      inherits: addresses_base
      fields:
        []
  - phone:
      tags: [view_type]
      inherits: phones_base
      fields:
        []
  - contact_group:
      tags: [view_type]
      inherits: contact_groups_base
      fields:
      - members:
          type: contact[]
          references: contact_group_member.contact_group_id
  - contact_group_member:
      tags: [datasource_type, many_to_many]
      inherits: base
      fields:
      - contact_id:
          references: contacts_base.id
      - contact_group_id:
          references: contact_groups_base.id
  - legacy_contact:
      tags: [datasource_type, view_type]
      fields:
      - key:
          type: string
          size: 64
      - first_name:
          type: string
          size: 128
      - last_name:
          type: string
          size: 128
      - email:
          type: string
          size: 256
          is_nullable: true
      - imported_at:
          type: datetime
          is_nullable: true
  - contact_change_log:
      tags: [datasource_type, view_type]
      inherits: set
      fields:
      - occurred_at:
          type: datetime
      - contact_id:
          type: number
      - changed_by:
          type: string
          size: 128
      - change_summary:
          type: string
          size: unlimited
  - import_contacts_request:
      tags: [datasource_type, view_type]
      fields:
      - source:
          type: string
          size: 64
      - payload:
          type: string
          size: unlimited
  - import_contacts_result:
      tags: [datasource_type, view_type]
      fields:
      - imported_count:
          type: number
      - skipped_count:
          type: number
      - failed_count:
          type: number
  - legacy_migration_result:
      tags: [datasource_type, view_type]
      fields:
      - migrated_count:
          type: number
      - skipped_count:
          type: number
      - completed_at:
          type: datetime
          is_nullable: true
`;

const datasourceYaml = `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - contact_source:
      fields:
        - name:
            is_unique: true
  - contacts_base:
      mapping: contacts
  - addresses_base:
      mapping: addresses
  - phones_base:
      mapping: phones
  - contact_groups_base:
      mapping: contact_groups
      fields:
        - name:
            is_unique: true
  - legacy_contact:
      mapping: OldContactsTbl
      fields:
        - key:
            is_fixed_id: true
            is_readonly: true
            is_unique: true
            mapping: CntID
        - first_name:
            mapping: FirstNm
        - last_name:
            mapping: LastNm
        - email:
            mapping: EmailAddr
        - imported_at:
            mapping: ImpDate
`;

const contactsFiles = (): Record<string, string> => ({ "types.yaml": typesYaml, "datasource.yaml": datasourceYaml });

const request = (dialect: Dialect, settings: Record<string, string> = { "backend.datasources": "sqlite" }) => ({
  dialect,
  reader: memoryReader(contactsFiles()),
  settings,
});

const textOf = (entries: { kind: string; filename: string; contents?: string }[], filename: string): string => {
  const entry = entries.find((item) => item.filename.endsWith(filename));
  if (entry === undefined || entry.kind !== "content" || entry.contents === undefined) throw new Error(`missing ${filename}`);
  return entry.contents;
};

const mem = (files: Record<string, string> = {}): { files: Record<string, string>; fs: Fs } => {
  const store = { ...files };
  return {
    files: store,
    fs: {
      readFile: async (path) => {
        const text = store[path];
        if (text === undefined) throw new Error("ENOENT");
        return text;
      },
      writeFile: async (path, text) => {
        store[path] = text;
      },
      readdir: async () => [],
    },
  };
};

const silent: Prompter = {
  select: async ({ default: def }) => def,
  checkbox: async ({ choices }) => choices.filter((choice) => choice.checked).map((choice) => choice.value),
  input: async ({ default: def }) => def,
};

const deps = (fs: Fs): Deps & { logs: string[] } => {
  const logs: string[] = [];
  return { fs, run: async () => undefined, log: (line) => logs.push(line), err: () => undefined, prompt: silent, logs };
};

describe("generate", () => {
  it("flattens settings", () => {
    expect(flattenSettings("settings:\n  backend:\n    datasources: sqlite\n  names:\n    - a\n    - b\n  on: false\n  empty: null\n")).toEqual({
      "backend.datasources": "sqlite",
      names: "a,b",
      on: "false",
      empty: "null",
    });
    expect(flattenSettings("backend:\n  datasources: mysql\n")).toEqual({ "backend.datasources": "mysql" });
    expect(flattenSettings("plain")).toEqual({});
    expect(flattenSettings("- a\n- b\n")).toEqual({});
    expect(flattenSettings("null")).toEqual({});
    expect(settingsFrom(undefined)).toEqual({ "backend.datasources": "sqlite" });
    expect(settingsFrom("count: 2\n")).toEqual({ "backend.datasources": "sqlite", count: "2" });
  });

  it("writes content and patches", async () => {
    const { files, fs } = mem({ [join("out", "e.txt")]: "keep\n// <region>\nold\n// </region>\n" });
    await writeEntries(fs, "out", [
      content("a.txt", "hello", { module: "a" }),
      patch("b.txt", "line\n"),
      patch("b.txt", "line\n"),
      patch("c.txt", "start", undefined, "Start"),
      patch("c.txt", "end", undefined, "None"),
      patch("d.txt", "tail\n", undefined, "End"),
      patch("e.txt", "next", "region"),
      patch("f.txt", "added", "region"),
    ]);
    expect(files[join("out", "a.txt")]).toBe("hello");
    expect(files[join("out", "b.txt")]).toBe("line\n");
    expect(files[join("out", "c.txt")]).toBe("start");
    expect(files[join("out", "d.txt")]).toBe("tail\n");
    expect(files[join("out", "e.txt")]).toContain("next");
    expect(files[join("out", "e.txt")]).not.toContain("old");
    expect(files[join("out", "f.txt")]).toContain("// <region>");
  });

  it("rejects an unknown provider", () => {
    expect(providerByName("deterministic").name).toBe("deterministic");
    expect(() => providerByName("missing")).toThrow(/Unknown provider: missing/);
  });

  it("maps legacy contacts and enriches contact lists", async () => {
    const csharp = await deterministic.generateRepositories(request("csharp"));
    const typescript = await deterministic.generateRepositories(request("typescript"));
    const rust = await deterministic.generateRepositories(request("rust"));
    const legacyCs = textOf(csharp, "LegacyContactRepository.cs");
    const contactCs = textOf(csharp, "ContactRepository.cs");
    const legacyTs = textOf(typescript, "legacy-contact-repository.ts");
    const contactTs = textOf(typescript, "contact-repository.ts");
    const legacyRs = textOf(rust, "legacy_contact_repository.rs");
    const contactRs = textOf(rust, "contact_repository.rs");
    expect(legacyCs).toContain('["legacy_contact"] = "OldContactsTbl"');
    expect(legacyCs).toContain('["Key"] = "CntID"');
    expect(legacyCs).toContain('["FirstName"] = "FirstNm"');
    expect(legacyCs).toContain('Ids = ["Key"]');
    expect(legacyCs).toContain('"Key"');
    expect(contactCs).toContain("AddEnriched");
    expect(contactCs).toContain('"Addresses"');
    expect(contactCs).toContain('"Phones"');
    expect(contactCs).toContain("EnrichmentPropertyType.List");
    expect(contactCs).toContain('"Version"');
    expect(textOf(csharp, "ContactGroupRepository.cs")).toContain("AddLinked");
    expect(csharp.some((entry) => entry.filename.includes("BaseRepository"))).toBe(false);
    expect(textOf(csharp, "ContactSourceRepository.cs")).toContain("ContactSource");
    expect(textOf(csharp, "ImportContactsRequestRepository.cs")).toContain("Ids = []");
    expect(legacyTs).toContain('["legacy_contact", "OldContactsTbl"]');
    expect(legacyTs).toContain('["key", "CntID"]');
    expect(legacyTs).toContain('ids: ["key"]');
    expect(contactTs).toContain('destinationProperty: "addresses"');
    expect(contactTs).toContain('destinationProperty: "phones"');
    expect(contactTs).toContain('type: "list"');
    expect(textOf(typescript, "contact-group-repository.ts")).toContain("addLinked");
    expect(legacyRs).toContain('"OldContactsTbl"');
    expect(legacyRs).toContain('"key".into(), "CntID".into()');
    expect(legacyRs).toContain('vec!["key".into()]');
    expect(contactRs).toContain("EnrichmentPropertyType::List");
    expect(contactRs).toContain('"addresses"');
    expect(textOf(rust, "contact_group_repository.rs")).toContain("add_linked");
    expect(textOf(rust, "mod.rs")).toContain("pub mod contact_repository;");
  });

  it("emits a typescript Contact type and sqlite tables", async () => {
    const types = await deterministic.generateTypes(request("typescript"));
    expect(types.some((entry) => entry.kind === "content" && entry.contents.includes("export interface Contact"))).toBe(true);
    const sql = await deterministic.generateDatasource(request("typescript"));
    const script = sql.filter((entry) => entry.kind === "content").map((entry) => entry.contents).join("\n");
    expect(script).toMatch(/CREATE TABLE/);
    expect(script).toContain("contacts");
    expect(script).toContain("OldContactsTbl");
    expect(script).toContain("CntID");
  });

  it("skips an abstract parent and rejects a repository cycle", async () => {
    const edge = {
      "types.yaml": `version: 1.0.0
types:
  - base:
      tags: [datasource_type]
      fields:
      - id:
          type: integer
          is_id: true
  - mapped_parent:
      tags: [datasource_type]
      inherits: base
      fields:
      - name:
          type: string
  - mapped_child:
      tags: [datasource_type]
      inherits: mapped_parent
      fields:
      - note:
          type: string
  - labeled:
      tags: [view_type]
      inherits: mapped_child
      mapping:
        missing: gone
      fields: []
  - abstract_ref:
      tags: [datasource_type]
      fields:
      - id:
          type: integer
          is_id: true
  - uses_ref:
      tags: [datasource_type]
      inherits: abstract_ref
      fields:
      - other_id:
          references: abstract_ref.id
  - pointed:
      tags: [datasource_type]
      fields:
      - id:
          type: integer
          is_id: true
      - name:
          type: string
  - pointer:
      tags: [datasource_type]
      fields:
      - id:
          type: integer
          is_id: true
      - pointed_id:
          references: pointed.id
      - loose:
          references: nowhere
  - joined:
      tags: [view_type]
      inherits: pointer
      union: [pointed]
      mapping:
        absent: gone
        pointed.name: pointed_name
      remove_fields: [pointed.id]
      fields: []
  - solo:
      tags: [datasource_type, many_to_many]
      fields:
      - parent_id:
          references: pointer.id
  - bag:
      tags: [datasource_type, view_type]
      fields:
      - label:
          type: string
      - items:
          type: pointer[]
          references: pointer.id
      - bare:
          type: pointer[]
      - ghosts:
          type: ghost[]
          references: ghost.ghost_id
      - solos:
          type: solo[]
          references: solo.parent_id
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - mapped_parent:
      mapping: mapped_parents
`,
    };
    const spec = await DeterministicParser(memoryReader(edge)).parse({ "backend.datasources": "sqlite" });
    const models = planRepositories(spec, "csharp", { "backend.datasources": "sqlite" });
    expect(models.some((model) => model.specName === "base")).toBe(false);
    expect(models.some((model) => model.specName === "mapped_parent")).toBe(true);
    expect(models.some((model) => model.specName === "abstract_ref")).toBe(true);
    const cycle = {
      "types.yaml": `version: 1.0.0
types:
  - left:
      tags: [datasource_type, view_type]
      fields:
      - id:
          type: integer
          is_id: true
      - rights:
          type: right[]
          references: right.left_id
  - right:
      tags: [datasource_type, view_type]
      fields:
      - id:
          type: integer
          is_id: true
      - lefts:
          type: left[]
          references: left.right_id
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    };
    const cycled = await DeterministicParser(memoryReader(cycle)).parse({});
    expect(() => planRepositories(cycled, "typescript", {})).toThrow(/repository cycle/);
  });

  it("writes each generate target", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    const targets = ["all", "types", "repositories", "datasource"] as const;
    for (const target of targets) {
      const store = mem(seeded);
      const io = deps(store.fs);
      expect(await run(["generate", target, "--dialect", "typescript", "--cwd", cwd], io, cwd)).toBe(0);
      expect(io.logs.length).toBeGreaterThan(0);
    }
    const absolute = mem({
      [join("/tmp/spec", "types.yaml")]: typesYaml,
      [join("/tmp/spec", "datasource.yaml")]: datasourceYaml,
    });
    const io = deps(absolute.fs);
    expect(await run(["generate", "repositories", "--dialect", "rust", "--input", "/tmp/spec", "--output", "/tmp/out"], io, cwd)).toBe(0);
    expect(absolute.files[join("/tmp/out", "rust/repositories/legacy_contact_repository.rs")]).toContain("CntID");
    const missing = mem();
    expect(await run(["generate", "types", "--dialect", "csharp", "--cwd", cwd], deps(missing.fs), cwd)).toBe(1);
    expect(await run(["generate", "types", "--dialect", "typescript", "--provider", "missing", "--cwd", cwd], deps(mem(seeded).fs), cwd)).toBe(1);
  });
});
