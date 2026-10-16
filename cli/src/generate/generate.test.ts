import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
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
import { pluralCamel, pluralPascal, pluralSnake } from "./RepositoryModel.ts";
import { providerByName } from "./providers.ts";
import { flattenSettings, settingsFrom } from "./settingsFlat.ts";
import { writeEntries } from "./writeEntries.ts";

const specDir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/contacts-crud-enriched-medium-rest-shared/deterministic");
const typesYaml = readFileSync(join(specDir, "types.yaml"), "utf8");
const datasourceYaml = readFileSync(join(specDir, "datasource.yaml"), "utf8");
const contactsFiles = (): Record<string, string> => ({ "types.yaml": typesYaml, "datasource.yaml": datasourceYaml });

const request = (
  dialect: Dialect,
  settings: Record<string, string> = { "backend.datasources": "sqlite", "languages.typescript.casing.fields": "camel" },
) => ({
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
  it("pluralizes the last word of an entity name", () => {
    expect(pluralPascal("Contact")).toBe("Contacts");
    expect(pluralPascal("ContactGroup")).toBe("ContactGroups");
    expect(pluralPascal("Address")).toBe("Addresses");
    expect(pluralPascal("Child")).toBe("Children");
    expect(pluralPascal("Person")).toBe("People");
    expect(pluralPascal("City")).toBe("Cities");
    expect(pluralPascal("Box")).toBe("Boxes");
    expect(pluralPascal("News")).toBe("News");
    expect(pluralPascal("Sheep")).toBe("Sheep");
    expect(pluralPascal("Contacts")).toBe("Contacts");
    expect(pluralPascal("Cities")).toBe("Cities");
    expect(pluralPascal("Boxes")).toBe("Boxes");
    expect(pluralPascal("People")).toBe("People");
    expect(pluralPascal("Church")).toBe("Churches");
    expect(pluralPascal("Dish")).toBe("Dishes");
    expect(pluralPascal("Buzz")).toBe("Buzzes");
    expect(pluralPascal("Class")).toBe("Classes");
    expect(pluralPascal("Bus")).toBe("Buses");
    expect(pluralPascal("Fish")).toBe("Fish");
    expect(pluralPascal("_")).toBe("_s");
    expect(pluralSnake("contact_group")).toBe("contact_groups");
    expect(pluralCamel("ContactGroup")).toBe("contactGroups");
  });

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
    expect(settingsFrom(undefined)).toEqual({
      "backend.datasources": "sqlite",
      "languages.typescript.casing.fields": "camel",
    });
    expect(settingsFrom("count: 2\n")).toEqual({
      "backend.datasources": "sqlite",
      "languages.typescript.casing.fields": "camel",
      count: "2",
    });
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
    expect(legacyCs).toContain('["LegacyContact"] = "OldContactsTbl"');
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
    expect(csharp.some((entry) => entry.filename.includes("ImportContactsRequestRepository"))).toBe(false);
    expect(contactCs).toContain("EnrichmentPropertyType.Dictionary");
    expect(contactCs).toContain('"Settings"');
    expect(contactCs).toContain('"Prefs"');
    expect(legacyTs).toContain('["LegacyContact", "OldContactsTbl"]');
    expect(contactTs).toContain('entity: "Contact"');
    expect(contactTs).toContain('["Contact", "contacts"]');
    expect(legacyTs).toContain('["key", "CntID"]');
    expect(legacyTs).toContain('["firstName", "FirstNm"]');
    expect(legacyTs).toContain('ids: ["key"]');
    expect(contactTs).toContain('["firstName", "first_name"]');
    expect(contactTs).toContain('destinationProperty: "addresses"');
    expect(contactTs).toContain('destinationProperty: "phones"');
    expect(contactTs).toContain('destinationProperty: "settings"');
    expect(contactTs).toContain('destinationProperty: "prefs"');
    expect(contactTs).toContain('destinationProperty: "contactSourceName"');
    expect(contactTs).toContain('type: "list"');
    expect(contactTs).toContain('type: "dictionary"');
    expect(textOf(typescript, "contact-group-repository.ts")).toContain("addLinked");
    expect(legacyRs).toContain('"LegacyContact"');
    expect(legacyRs).toContain('"OldContactsTbl"');
    expect(contactRs).toContain('"Contact"');
    expect(legacyRs).toContain('"key".into(), "CntID".into()');
    expect(legacyRs).toContain('vec!["key".into()]');
    expect(contactRs).toContain("EnrichmentPropertyType::List");
    expect(contactRs).toContain("EnrichmentPropertyType::Dictionary");
    expect(contactRs).toContain('"addresses"');
    expect(contactRs).toContain('"settings"');
    expect(contactRs).toContain('"prefs"');
    expect(contactRs).toContain("pub struct ContactRepository");
    expect(contactRs).toContain("pub address: Arc<super::address_repository::AddressRepository>");
    expect(contactRs).toContain("pub phone: Arc<super::phone_repository::PhoneRepository>");
    expect(contactRs).toContain("pub contact_source: Arc<CRUDRepository>");
    expect(contactRs).toContain("Arc::clone(&address.repo)");
    expect(textOf(rust, "contact_group_repository.rs")).toContain("add_linked");
    expect(textOf(rust, "contact_group_repository.rs")).toContain("pub struct ContactGroupRepository");
    expect(textOf(rust, "mod.rs")).toContain("pub mod contact_repository;");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain("ModuleInitializer");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain("RepositoryFactory.Bind(Register)");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain('"Contact", "Contacts"');
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain('"ContactGroup", "ContactGroups"');
    expect(textOf(csharp, "ContactRepository.cs")).toContain("CreateReadonly");
    expect(textOf(typescript, "repository-registry.ts")).toContain("RepositoryFactory.bind(register)");
    expect(textOf(typescript, "repository-registry.ts")).toContain('"Contact", "contacts"');
    expect(textOf(typescript, "repository-registry.ts")).toContain('"ContactGroup", "contactGroups"');
    expect(contactTs).toContain("createReadonly");
    expect(contactTs.slice(contactTs.indexOf("createReadonly"))).not.toContain("ids:");
    expect(legacyTs.slice(legacyTs.indexOf("createReadonly"))).not.toContain("ids:");
    expect(contactTs).not.toMatch(/from "[^"]+\.ts"/);
    expect(legacyTs).not.toMatch(/from "[^"]+\.ts"/);
    expect(textOf(typescript, "repository-registry.ts")).not.toMatch(/from "[^"]+\.ts"/);
    expect(textOf(rust, "repository_registry.rs")).toContain("RepositoryFactory::bind(register)");
    expect(textOf(rust, "repository_registry.rs")).toContain("#[ctor::ctor]");
    expect(textOf(rust, "repository_registry.rs")).not.toContain("Once");
    expect(textOf(rust, "repository_registry.rs")).not.toContain("bind();\n");
    expect(textOf(rust, "repository_registry.rs")).toContain('"Contact"');
    expect(textOf(rust, "repository_registry.rs")).toContain('"contacts"');
    expect(textOf(rust, "repository_registry.rs")).toContain('"contact_groups"');
    expect(textOf(rust, "repository_registry.rs")).toContain("fn contacts(&self)");
    expect(textOf(rust, "repository_registry.rs")).toContain("fn contact_groups(&self)");
    expect(textOf(rust, "mod.rs")).toContain("pub mod repository_registry;");
  });

  it("imports kebab type files and omits ids from createReadonly", async () => {
    const typescript = await deterministic.generateRepositories(request("typescript", {
      "backend.datasources": "sqlite",
      "languages.typescript.casing.fields": "camel",
      "languages.typescript.casing.file_names": "kebab",
    }));
    const addressType = textOf(typescript, "address-type-repository.ts");
    expect(addressType).toContain('from "../types/generated/datasource/address-type"');
    expect(addressType).not.toContain("addressType");
    expect(addressType).not.toMatch(/from "[^"]+\.ts"/);
    expect(addressType.slice(addressType.indexOf("createReadonly"))).not.toContain("ids:");
    expect(addressType.slice(0, addressType.indexOf("createReadonly"))).toContain("ids:");
  });

  it("emits a typescript Contact type and sqlite tables", async () => {
    const types = await deterministic.generateTypes(request("typescript"));
    const text = types.filter((entry) => entry.kind === "content").map((entry) => entry.contents).join("\n");
    expect(text).toContain("export interface Contact");
    expect(text).toContain("firstName");
    expect(text).toContain("contactSourceName");
    const sql = await deterministic.generateDatasource(request("typescript"));
    await deterministic.generateDatasource(request("typescript", {}));
    await deterministic.generateDatasource(request("typescript", { "backend.datasources": "sqlite, sqlite,," }));
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
      - missing_map:
          type: ghost{}
      - no_owner:
          type: empty_dict{}
      - keyed:
          type: keyed_dict{}
  - empty_dict:
      tags: [datasource_type]
      inherits: dictionary
      fields:
      - key:
          type: string
  - keyed_dict:
      tags: [datasource_type]
      inherits: dictionary
      fields:
      - bag_id:
          references: bag.id
      - key:
          type: string
  - solo_card:
      tags: [view_type]
      inherits: solo
      fields: []
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
    const contacts = await DeterministicParser(memoryReader(contactsFiles())).parse({ "backend.datasources": "sqlite" });
    const withoutValue = (name: string) => (type: (typeof contacts.types)[number]) =>
      type.name === name ? { ...type, fields: type.fields.filter((field) => field.name !== "value") } : type;
    planRepositories(
      {
        ...contacts,
        types: contacts.types.map(withoutValue("contact_settings")),
        expandedTypes: contacts.expandedTypes.map(withoutValue("contact_settings")),
      },
      "typescript",
      { "backend.datasources": "sqlite" },
    );
    planRepositories(
      {
        ...contacts,
        expandedTypes: contacts.expandedTypes.filter((type) => type.name !== "locale_preferences"),
      },
      "typescript",
      { "backend.datasources": "sqlite" },
    );
    const models = planRepositories(spec, "csharp", { "backend.datasources": "sqlite" });
    planRepositories(
      {
        ...spec,
        types: spec.types.filter((type) => type.name !== "solo"),
        expandedTypes: spec.expandedTypes.filter((type) => type.name !== "solo"),
      },
      "csharp",
      { "backend.datasources": "sqlite" },
    );
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
    const rustTypes = mem(seeded);
    const rustIo = deps(rustTypes.fs);
    expect(await run(["generate", "types", "--dialect", "rust", "--cwd", cwd], rustIo, cwd)).toBe(0);
    expect(rustIo.logs.length).toBeGreaterThan(0);
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

  it("defaults missing datasource.yaml to tagged types and one view repository", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const store = mem({
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - note:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - title:
            type: string
  - note_card:
      tags: [view_type]
      inherits: note
      fields: []
`,
    });
    const io = deps(store.fs);
    expect(await run(["generate", "repositories", "--dialect", "typescript", "--cwd", cwd], io, cwd)).toBe(0);
    const registry = store.files[join(cwd, "generated/typescript/repositories/repository-registry.ts")];
    expect(registry).toContain('"NoteCard", "noteCards"');
    expect(registry).not.toContain('"Note"');
    expect(store.files[join(cwd, "generated/typescript/repositories/note-card-repository.ts")]).toContain('["NoteCard", "notes"]');
    expect(store.files[join(cwd, "generated/typescript/repositories/note-repository.ts")]).toBeUndefined();
  });

  it("keeps an explicit datasource.yaml filter", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const store = mem({
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - kept:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - dropped:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
`,
      [join(input, "datasource.yaml")]: `version: 1.0.0
includes:
  - types:
      filter: type == "kept"
types: []
`,
    });
    const io = deps(store.fs);
    expect(await run(["generate", "repositories", "--dialect", "typescript", "--cwd", cwd], io, cwd)).toBe(0);
    const registry = store.files[join(cwd, "generated/typescript/repositories/repository-registry.ts")];
    expect(registry).toContain('"Kept", "kepts"');
    expect(registry).not.toContain('"Dropped"');
  });

  it("writes migrations create into the migraters layout", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    const store = mem(seeded);
    const io = deps(store.fs);
    expect(await run(["migrations", "create", "--cwd", cwd], io, cwd)).toBe(0);
    const up = store.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")];
    expect(up).toMatch(/CREATE TABLE/i);
    const dry = mem(seeded);
    const dryIo = deps(dry.fs);
    expect(await run(["migrations", "create", "--cwd", cwd, "--dry-run"], dryIo, cwd)).toBe(0);
    expect(dry.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toBeUndefined();
    expect(dryIo.logs.some((line) => line.endsWith("sql/sqlite/migrations/0001_initial_up.sql"))).toBe(true);
    const pathStore = mem(seeded);
    const pathIo = deps(pathStore.fs);
    expect(await run(["migrations", "create", "--cwd", cwd, "--migrations-path", "db/changes"], pathIo, cwd)).toBe(0);
    expect(pathStore.files[join(cwd, "db/changes/0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
    const dual = mem({
      ...seeded,
      [join(input, "settings.yaml")]: "settings:\n  backend:\n    datasources: sqlite,postgres\n",
    });
    expect(await run(["migrations", "create", "--cwd", cwd, "--migrations-path", "db/changes"], deps(dual.fs), cwd)).toBe(1);
    const absolute = mem({
      [join("/tmp/spec", "types.yaml")]: typesYaml,
      [join("/tmp/spec", "datasource.yaml")]: datasourceYaml,
    });
    expect(
      await run(
        ["migrations", "create", "--input", "/tmp/spec", "--migrations-path", "/tmp/mig"],
        deps(absolute.fs),
        cwd,
      ),
    ).toBe(0);
    expect(absolute.files[join("/tmp/mig", "0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
  });
});
