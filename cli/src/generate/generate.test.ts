import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse, stringify } from "yaml";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { content, patch } from "@deterministic-code/generators-common/generate-entry";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { describe, expect, it } from "vitest";
import type { Deps, Fs } from "../io.ts";
import { parseArgs } from "../parse.ts";
import type { Prompter } from "../setup.ts";
import { applyGenerate } from "./applyGenerate.ts";
import { applyMigrations } from "./applyMigrations.ts";
import { applyMigrateUp } from "./migrateUp.ts";
import { sqlFlagsOf } from "./sqlFlags.ts";
import { deterministic } from "./DeterministicProvider.ts";
import type { Dialect } from "./GenerateRequest.ts";
import { planRepositories } from "./RepositoryPlan.ts";
import { csharpRepositoryGenerators } from "./CSharpRepositoryGenerators.ts";
import { rustRepositoryGenerators } from "./RustRepositoryGenerators.ts";
import { typescriptRepositoryGenerators } from "./TypeScriptRepositoryGenerators.ts";
import { providerByName } from "./providers.ts";
import { flattenSettings, settingsFrom } from "./settingsFlat.ts";
import { deleteStale, writeEntries } from "./writeEntries.ts";
import { defaultDatasourceYaml, loadSpec } from "./loadSpec.ts";
import { nodeFs } from "../io.ts";
import { DictionaryMapper } from "@deterministic-code/datasources";
import { register } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";

register("sqlite", new SqliteDriver());

const specDir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/contacts-crud-enriched-medium-rest-shared/deterministic");
const userRolesDir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/user-roles-simple-cli-shared/deterministic");
const specFile = (name: string): string => readFileSync(join(specDir, name), "utf8");
const typeEntries = (text: string): string => {
  const i = text.search(/\ntypes:\n/);
  return i === -1 ? "" : text.slice(i + "\ntypes:\n".length).replace(/\n+$/, "");
};
const typesYaml = `version: 1.0.0\ntypes:\n${typeEntries(specFile("base.yaml"))}\n${typeEntries(specFile("lookups.yaml"))}\n${typeEntries(specFile("types.yaml"))}\n`;
const sqliteDatasourceYaml = specFile("datasource.sqlite.yaml");
const mysqlDatasourceYaml = specFile("datasource.mysql.yaml");
const postgresDatasourceYaml = specFile("datasource.postgres.yaml");
const userRolesFiles = (): Record<string, string> => ({
  "types.yaml": readFileSync(join(userRolesDir, "types.yaml"), "utf8"),
  "datasource.yaml": readFileSync(join(userRolesDir, "datasource.yaml"), "utf8"),
});
const contactsDatasources = (): Record<string, string> => ({
  "datasource.yaml": sqliteDatasourceYaml,
  "datasource.sqlite.yaml": sqliteDatasourceYaml,
  "datasource.mysql.yaml": mysqlDatasourceYaml,
  "datasource.postgres.yaml": postgresDatasourceYaml,
});
const contactsAuthored = (): Record<string, string> => ({
  "types.yaml": specFile("types.yaml"),
  "base.yaml": specFile("base.yaml"),
  "lookups.yaml": specFile("lookups.yaml"),
  ...contactsDatasources(),
});
const contactsFiles = (): Record<string, string> => ({
  "types.yaml": typesYaml,
  ...contactsDatasources(),
});

const request = (
  dialect: Dialect,
  settings: Record<string, string> = { "backend.datasources": "sqlite", "languages.typescript.casing.fields": "camel" },
  env: Record<string, string> = { INSTALLED_PROVIDERS: "sqlite" },
) => ({
  dialect,
  reader: memoryReader(contactsFiles()),
  settings,
  env,
});

const textOf = (entries: { kind: string; filename: string; contents?: string }[], filename: string): string => {
  const entry = entries.find((item) => item.filename.endsWith(filename));
  if (entry === undefined || entry.kind !== "content" || entry.contents === undefined) throw new Error(`missing ${filename}`);
  return entry.contents;
};

const fieldType = (text: string, field: string): string => {
  const match =
    text.match(new RegExp(`public (\\S+) ${field} `)) ??
    text.match(new RegExp(`\\b${field}[!?]?: ([^;\\n]+)`)) ??
    text.match(new RegExp(`pub ${field}: ([^,;\\n]+)`));
  if (match === null || match[1] === undefined) throw new Error(`missing ${field}`);
  return match[1].trim();
};

const mem = (files: Record<string, string> = {}): { files: Record<string, string>; fs: Fs } => {
  const store = { ...files };
  return {
    files: store,
    fs: {
      readFile: async (path) => {
        const text = store[path] ?? store[relative(process.cwd(), resolve(path))];
        if (text === undefined) throw new Error("ENOENT");
        return text;
      },
      writeFile: async (path, text) => {
        store[path] = text;
      },
      readdir: async (path) => {
        const prefix = path.endsWith("/") ? path : `${path}/`;
        const names = new Set<string>();
        for (const file of Object.keys(store)) {
          if (!file.startsWith(prefix)) continue;
          const name = file.slice(prefix.length).split("/")[0];
          if (name) names.add(name);
        }
        return [...names];
      },
      unlink: async (path) => {
        delete store[path];
      },
    },
  };
};

const silent: Prompter = {
  select: async ({ default: def }) => def,
  checkbox: async ({ choices }) => choices.filter((choice) => choice.checked).map((choice) => choice.value),
  input: async ({ default: def }) => def,
  confirm: async ({ default: def }) => def,
};

const deps = (fs: Fs): Deps & { logs: string[] } => {
  const logs: string[] = [];
  return { fs, run: async () => undefined, log: (line) => logs.push(line), err: () => undefined, prompt: silent, logs };
};

const exec = async (argv: string[], io: Deps, cwd = "."): Promise<number> => {
  try {
    const { command, flags } = parseArgs(argv, cwd);
    if (command.kind === "migrations-create") {
      await applyMigrations(command, flags, io);
      return 0;
    }
    if (command.kind !== "generate") throw new Error(command.kind);
    await applyGenerate(command, flags, io);
    if (command.target !== "all") return 0;
    if (command.migrations)
      await applyMigrations(
        {
          kind: "migrations-create",
          provider: command.provider,
          input: command.input,
          storedProcedures: command.storedProcedures,
          migrationsRoot: command.migrationsRoot,
          ...sqlFlagsOf(command),
        },
        flags,
        io,
      );
    if (command.migrate) await applyMigrateUp(command, flags, io);
    return 0;
  } catch (error) {
    io.err(error instanceof Error ? error.message : "Unknown error");
    return 1;
  }
};

describe("generate", () => {
  it("fails parse with missing type user before generate writes files", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const store = mem({
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - review:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - user_id:
            references: user.id
`,
      [join(input, "datasource.yaml")]: defaultDatasourceYaml,
    });
    const errs: string[] = [];
    const io = deps(store.fs);
    io.err = (line) => errs.push(line);
    expect(
      await exec(
        ["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--no-migrations", "--no-auto-migrate-up"],
        io,
        cwd,
      ),
    ).toBe(1);
    expect(errs).toEqual(["review.user_id references user.id has no type"]);
    expect(Object.keys(store.files).every((path) => !path.includes("generated") && !path.includes("sql/"))).toBe(true);
  });

  it("fails parse when complex-game types omit the user include", async () => {
    const dir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/complex-game-rest-shared/deterministic");
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const types = readFileSync(join(dir, "types.yaml"), "utf8").replace(/\nincludes:\n[\s\S]*?(?=\ntypes:\n)/, "\n");
    const store = mem({
      [join(input, "types.yaml")]: types,
      [join(input, "datasource.yaml")]: readFileSync(join(dir, "datasource.yaml"), "utf8"),
    });
    const errs: string[] = [];
    const io = deps(store.fs);
    io.err = (line) => errs.push(line);
    expect(
      await exec(
        ["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--no-migrations", "--no-auto-migrate-up"],
        io,
        cwd,
      ),
    ).toBe(1);
    expect(errs).toEqual(["user_settings.user_id references user_base.id has no type"]);
    expect(Object.keys(store.files).some((path) => path.includes("generated") || path.includes("sql/"))).toBe(false);
  });

  it("fails parse when a reference names a missing type or field", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const store = mem({
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - contact:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - address:
      tags: [datasource_type]
      fields:
        - ghosts:
            type: ghost[]
            references: ghost.ghost_id
`,
      [join(input, "datasource.yaml")]: defaultDatasourceYaml,
    });
    const errs: string[] = [];
    const io = deps(store.fs);
    io.err = (line) => errs.push(line);
    expect(
      await exec(
        ["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--no-migrations", "--no-auto-migrate-up"],
        io,
        cwd,
      ),
    ).toBe(1);
    expect(errs).toEqual(["address.ghosts references ghost.ghost_id has no type"]);
  });

  it("writes sqlite migrations for complex-game with local includes", async () => {
    const dir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/complex-game-rest-shared/deterministic");
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const store = mem({
      [join(input, "types.yaml")]: readFileSync(join(dir, "types.yaml"), "utf8"),
      [join(input, "user-roles-simple-cli-types.yaml")]: readFileSync(join(dir, "user-roles-simple-cli-types.yaml"), "utf8"),
      [join(input, "contacts-crud-enriched-medium-rest-types.yaml")]: readFileSync(
        join(dir, "contacts-crud-enriched-medium-rest-types.yaml"),
        "utf8",
      ),
      [join(input, "datasource.yaml")]: readFileSync(join(dir, "datasource.yaml"), "utf8"),
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\n",
    });
    expect(
      await exec(["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--no-auto-migrate-up"], deps(store.fs), cwd),
    ).toBe(0);
    const up = store.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")];
    expect(up).toMatch(/CREATE TABLE/i);
    expect(up).toMatch(/user_bases/i);
  });

  it("writes sqlite seed inserts from combined datasource_seeds includes", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const store = mem({
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - gender:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - phone_type:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
`,
      [join(input, "datasource.yaml")]: defaultDatasourceYaml,
      [join(input, "datasource_seeds.yaml")]: `version: 1.0.0
includes:
  - file: more.yaml
seeds:
  - phone_type:
      - id1:
          name: Mobile
`,
      [join(input, "more.yaml")]: `version: 1.0.0
seeds:
  - gender:
      - id1:
          name: Male
`,
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\n",
    });
    expect(
      await exec(["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--no-auto-migrate-up"], deps(store.fs), cwd),
    ).toBe(0);
    const up = store.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")];
    expect(up).toMatch(/INSERT INTO "genders"/);
    expect(up).toMatch(/Male/);
    expect(up).toMatch(/INSERT INTO "phone_types"/);
    expect(up).toMatch(/Mobile/);
  });

  it("generates the complex-game spec after resolving file includes", async () => {
    const dir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/complex-game-rest-shared/deterministic");
    const loaded = await loadSpec(nodeFs, dir);
    const spec = await DeterministicParser(memoryReader(loaded.files)).parse(loaded.settings);
    const names = spec.types.map((type) => type.name);
    expect(names).toEqual(expect.arrayContaining(["user_base", "user", "contact", "games", "categories", "user_settings"]));
    expect(spec.types.find((type) => type.name === "user")?.fields.map((field) => field.name)).toEqual(
      expect.arrayContaining(["settings", "roles", "groups"]),
    );
    expect(spec.types.find((type) => type.name === "contacts_base")?.fields.some((field) => field.name === "user_id")).toBe(true);
    const models = planRepositories(spec, "typescript", loaded.settings);
    expect(models.map((model) => model.specName)).toEqual(
      expect.arrayContaining(["user", "games", "contact", "categories", "reviews", "game_states"]),
    );
    const entries = await deterministic.generateTypes({
      dialect: "typescript",
      reader: memoryReader(loaded.files),
      settings: loaded.settings,
      env: { INSTALLED_PROVIDERS: "sqlite" },
    });
    expect(entries.length).toBeGreaterThan(0);
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
    const leftover = mem({
      [join("out", "keep.ts")]: "keep",
      [join("out", "typescript/types/generated/views/categories.ts")]: "old",
    });
    expect(await deleteStale(leftover.fs, "out", [join("out", "keep.ts")])).toEqual([
      join("out", "typescript/types/generated/views/categories.ts"),
    ]);
    expect(leftover.files[join("out", "keep.ts")]).toBe("keep");
    expect(leftover.files[join("out", "typescript/types/generated/views/categories.ts")]).toBeUndefined();
    expect(await deleteStale(mem().fs, "out", [])).toEqual([]);
  });

  it("keeps leftover generated files unless --delete", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const leftover = join(cwd, "generated/typescript/types/generated/views/categories.ts");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[leftover] = "old";
    const kept = mem(seeded);
    expect(
      await exec(
        ["generate", "types", "--dialect", "typescript", "--cwd", cwd, "--no-migrations", "--no-auto-migrate-up"],
        deps(kept.fs),
        cwd,
      ),
    ).toBe(0);
    expect(kept.files[leftover]).toBe("old");
    const removed = mem(seeded);
    expect(
      await exec(
        ["generate", "types", "--dialect", "typescript", "--cwd", cwd, "--delete", "--no-migrations", "--no-auto-migrate-up"],
        deps(removed.fs),
        cwd,
      ),
    ).toBe(0);
    expect(removed.files[leftover]).toBeUndefined();
    expect(Object.keys(removed.files).some((path) => path.endsWith("contact.ts"))).toBe(true);
  });

  it("rejects an unknown provider", () => {
    expect(providerByName("deterministic").name).toBe("deterministic");
    expect(() => providerByName("missing")).toThrow(/Unknown provider: missing/);
  });

  it("emits relatedProperty on generated links when the related type has name", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - user:
      tags: [datasource_type, view_type]
      fields:
        - id:
            type: integer
            is_id: true
        - roles:
            type: role[]
            references: user_role.user_id
        - groups:
            type: group[]
            references: user_group.user_id
  - role:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - group:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - user_role:
      tags: [datasource_type, many_to_many]
      fields:
        - user_id:
            references: user.id
        - role_id:
            references: role.id
  - user_group:
      tags: [datasource_type, many_to_many]
      fields:
        - user_id:
            references: user.id
        - group_id:
            references: group.id
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const csharpPlan = planRepositories(spec, "csharp", settings);
    const typescriptPlan = planRepositories(spec, "typescript", { ...settings, "languages.typescript.casing.fields": "camel" });
    const rustPlan = planRepositories(spec, "rust", settings);
    expect(csharpPlan.find((model) => model.specName === "user")?.enrichments.find((item) => item.destination === "Roles")?.relatedProperty).toBe("Name");
    expect(typescriptPlan.find((model) => model.specName === "user")?.enrichments.find((item) => item.destination === "roles")?.relatedProperty).toBe("name");
    expect(rustPlan.find((model) => model.specName === "user")?.enrichments.find((item) => item.destination === "roles")?.relatedProperty).toBe("name");
    expect(csharpPlan.find((model) => model.specName === "user")?.enrichments.find((item) => item.destination === "Groups")?.relatedProperty).toBeUndefined();
    expect(typescriptPlan.find((model) => model.specName === "user")?.enrichments.find((item) => item.destination === "groups")?.relatedProperty).toBeUndefined();
    expect(rustPlan.find((model) => model.specName === "user")?.enrichments.find((item) => item.destination === "groups")?.relatedProperty).toBeUndefined();
    const csharp = await deterministic.generateRepositories({ dialect: "csharp", reader: memoryReader(files), settings });
    const typescript = await deterministic.generateRepositories({
      dialect: "typescript",
      reader: memoryReader(files),
      settings: { ...settings, "languages.typescript.casing.fields": "camel" },
    });
    const rust = await deterministic.generateRepositories({ dialect: "rust", reader: memoryReader(files), settings });
    expect(csharp.some((entry) => entry.kind === "content" && entry.contents?.includes('AddLinked(new("Roles", "Id", "Id", "Name", Association: new("UserId", "RoleId"))'))).toBe(true);
    expect(typescript.some((entry) => entry.kind === "content" && entry.contents?.includes('relatedProperty: "name"'))).toBe(true);
    expect(rust.some((entry) => entry.kind === "content" && entry.contents?.includes('Some("name".into())'))).toBe(true);
  });

  it("reads a linked related type from expanded types and throws when that type is gone", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - user:
      tags: [datasource_type, view_type]
      fields:
        - id:
            type: integer
            is_id: true
        - roles:
            type: role[]
            references: user_role.user_id
  - role:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - user_role:
      tags: [datasource_type, many_to_many]
      fields:
        - user_id:
            references: user.id
        - role_id:
            references: role.id
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const withoutRoleTable = {
      ...spec,
      types: spec.types.filter((type) => type.name !== "role"),
      datasource: spec.datasource.filter((table) => table.name !== "role"),
    };
    const fromExpanded = planRepositories(withoutRoleTable, "csharp", settings);
    expect(fromExpanded.find((model) => model.specName === "user")?.enrichments.find((item) => item.destination === "Roles")?.relatedProperty).toBe("Name");
    const missing = {
      ...withoutRoleTable,
      expandedTypes: withoutRoleTable.expandedTypes.filter((type) => type.name !== "role"),
    };
    expect(() => planRepositories(missing, "csharp", settings)).toThrow("role: missing type");
    expect(() =>
      planRepositories(
        {
          ...spec,
          types: spec.types.map((type) =>
            type.name === "role" ? { ...type, fields: type.fields.filter((field) => field.name !== "id") } : type,
          ),
          expandedTypes: spec.expandedTypes.map((type) =>
            type.name === "role" ? { ...type, fields: type.fields.filter((field) => field.name !== "id") } : type,
          ),
        },
        "csharp",
        settings,
      ),
    ).toThrow("role: missing primary key");
  });

  it("maps legacy contacts and enriches contact lists", async () => {
    const csharp = await deterministic.generateRepositories(request("csharp"));
    const typescript = await deterministic.generateRepositories(request("typescript"));
    const rust = await deterministic.generateRepositories(request("rust"));
    const legacyCs = textOf(csharp, "LegacyContactsRepository.cs");
    const contactCs = textOf(csharp, "ContactsRepository.cs");
    const legacyTs = textOf(typescript, "legacyContactsRepository.ts");
    const contactTs = textOf(typescript, "contactsRepository.ts");
    const legacyRs = textOf(rust, "legacyContactsRepository.rs");
    const contactRs = textOf(rust, "contactsRepository.rs");
    expect(legacyCs).toContain('["LegacyContact"] = "OldContactsTbl"');
    expect(legacyCs).toContain('["Key"] = "CntID"');
    expect(legacyCs).toContain('["FirstName"] = "FirstNm"');
    expect(legacyCs).toContain('Ids = ["Key"]');
    expect(legacyCs).toContain('"Key"');
    expect(contactCs).toContain("AddEnriched");
    expect(contactCs).toContain('"Addresses"');
    expect(contactCs).toContain('"Phones"');
    expect(contactCs).toContain("new CRUDRepositoryBuilder<Backend.Types.View.Contact>()");
    expect(contactCs).toContain('.Add("mysql", new()');
    expect(contactCs).toContain('.Add("postgres", new()');
    expect(contactCs).toContain('CreateProperties = ["Uuid", "Version", "FirstName", "LastName", "Email", "Notes"]');
    expect(contactCs).toContain('CreateProperties = ["FirstName", "LastName", "Email", "Notes"]');
    expect(contactCs).toContain('UpdateProperties = ["Uuid", "Version", "FirstName", "LastName", "Email", "Notes"]');
    expect(contactCs).toContain(".For(connection)");
    expect(contactCs).toContain("EnrichmentPropertyType.List");
    expect(contactCs).toContain('"Version"');
    expect(textOf(csharp, "ContactGroupsRepository.cs")).toContain("AddLinked");
    expect(csharp.some((entry) => entry.filename.includes("BaseRepository"))).toBe(false);
    expect(textOf(csharp, "ContactSourcesRepository.cs")).toContain("ContactSource");
    expect(csharp.some((entry) => entry.filename.includes("ImportContactsRequestRepository"))).toBe(false);
    expect(contactCs).toContain("EnrichmentPropertyType.Dictionary");
    expect(contactCs).toContain('DictionaryKey: "Key"');
    expect(contactCs).toContain('"Settings"');
    expect(contactCs).toContain('"Prefs"');
    expect(legacyTs).toContain('["LegacyContact", "OldContactsTbl"]');
    expect(contactTs).toContain("new CRUDRepositoryBuilder<Contact>()");
    expect(contactTs).toContain('.add("mysql", {');
    expect(contactTs).toContain('.add("postgres", {');
    expect(contactTs).toContain('createProperties: ["uuid", "version", "firstName", "lastName", "email", "notes"]');
    expect(contactTs).toContain('createProperties: ["firstName", "lastName", "email", "notes"]');
    expect(contactTs).not.toContain("this.createZeros");
    expect(contactTs).toContain('firstName: { type: "string"');
    expect(contactTs).toContain('lastName: { type: "string"');
    expect(contactTs).toContain("hasDefault:");
    expect(contactTs).toContain(".for(connection)");
    expect(contactTs).toContain('entity: "Contact"');
    expect(contactTs).toContain('new DictionaryMapper([["Contact", "contacts"]])');
    expect(contactTs).not.toContain('["contactSourceName", "contact_source_name"]');
    expect(contactTs).toContain('["contactSourceId", "contact_source_id"]');
    expect(contactCs).toContain('["ContactSourceId"] = "contact_source_id"');
    expect(contactCs).not.toMatch(/CreateProperties = \[[^\]]*ContactSourceId/);
    expect(contactTs).toMatch(/\}\)\n\s*\.for\(connection\)/);
    for (const list of contactTs.match(/new DictionaryMapper\((\[(?:[^[\]]|\[[^[\]]*\])*\])\)/g) ?? []) {
      const pairs = new Function(`return ${list.slice("new DictionaryMapper(".length, -1)}`)() as [string, string][];
      expect(() => new DictionaryMapper(pairs)).not.toThrow();
    }
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
    expect(contactTs).toContain('dictionaryKey: "key"');
    expect(textOf(typescript, "contactGroupsRepository.ts")).toContain("addLinked");
    expect(legacyRs).toContain('"LegacyContact"');
    expect(legacyRs).toContain('"OldContactsTbl"');
    expect(contactRs).toContain('"Contact"');
    expect(legacyRs).toContain('"key".into(), "CntID".into()');
    expect(legacyRs).toContain('vec!["key".into()]');
    expect(contactRs).toContain("EnrichmentPropertyType::List");
    expect(contactRs).toContain("EnrichmentPropertyType::Dictionary");
    expect(contactRs).toContain('.with_dictionary_key("key")');
    expect(contactRs).toContain('"addresses"');
    expect(contactRs).toContain('"settings"');
    expect(contactRs).toContain('"prefs"');
    expect(contactRs).not.toContain("with_create_zeros");
    expect(contactRs).toContain("SchemaColumn");
    expect(contactRs).toContain("CRUDRepositoryBuilder::new()");
    expect(contactRs).toContain('.add_for("mysql"');
    expect(contactRs).toContain('.add_for("postgres"');
    expect(contactRs).not.toMatch(/read_properties = vec!\[[^\]]*contact_source_name/);
    expect(contactRs).toMatch(/read_properties = vec!\[[^\]]*contact_source_id/);
    expect(contactRs).not.toMatch(/create_properties = Some\(vec!\[[^\]]*contact_source_id/);
    expect(contactRs).not.toMatch(/update_properties = Some\(vec!\[[^\]]*contact_source_id/);
    expect(contactRs).not.toMatch(/create_properties = Some\(vec!\[[^\]]*contact_source_name/);
    expect(contactRs).not.toMatch(/update_properties = Some\(vec!\[[^\]]*contact_source_name/);
    expect(contactRs).toContain("for_connection(connection.as_ref())");
    expect(contactRs).toContain("pub struct ContactsRepository");
    expect(contactRs).toContain("async fn read_single_where");
    expect(contactRs).toContain("fn read_where");
    expect(contactRs).not.toContain("ReadInput");
    expect(contactRs).toContain("pub address: AddressesRepository");
    expect(contactRs).toContain("pub phone: PhonesRepository");
    expect(contactRs).toContain("pub contact_source: ContactSourcesRepository");
    expect(contactRs).not.toContain("Arc<CRUDRepository<Address>>");
    expect(contactRs).toContain("AddressesRepository::new");
    expect(contactRs).toContain("&address");
    expect(contactTs).toContain("readonly address: AddressesRepository");
    expect(contactCs).toContain("public AddressesRepository Address { get; }");
    expect(textOf(rust, "schema_objects.rs")).toContain("impl SchemaObject for Contact");
    expect(textOf(rust, "contactsRepository.rs")).toContain("Schema::new");
    expect(textOf(rust, "contactsRepository.rs")).toContain('"integer"');
    expect(textOf(rust, "contactsRepository.rs")).not.toContain("impl Entity");
    expect(textOf(rust, "phonesRepository.rs")).toContain("phone_type_name");
    expect(textOf(rust, "contactsRepository.rs")).toContain("contact_source_name");
    expect(textOf(rust, "contactGroupsRepository.rs")).toContain("add_linked");
    expect(textOf(rust, "contactGroupsRepository.rs")).toContain("pub struct ContactGroupsRepository");
    expect(textOf(rust, "mod.rs")).toContain("pub mod contactsRepository;");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain("ModuleInitializer");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain("RepositoryFactory.Bind(Register)");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain('"Contact", "Contacts"');
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain('"ContactGroup", "ContactGroups"');
    expect(textOf(csharp, "ContactsRepository.cs")).toContain("CreateReadonly");
    expect(textOf(csharp, "ContactsRepository.cs")).toContain("CRUDRepository<Backend.Types.View.Contact>");
    expect(textOf(csharp, "ContactSourcesRepository.cs")).toContain("ReadonlyRepository<");
    expect(textOf(typescript, "contactSourcesRepository.ts")).toContain("extends ReadonlyRepository<ContactSource>");
    expect(textOf(typescript, "contactSourcesRepository.ts")).not.toContain("extends CRUDRepository<ContactSource>");
    expect(textOf(typescript, "contactsRepository.ts")).toContain("extends CRUDRepository<Contact>");
    expect(textOf(rust, "contactSourcesRepository.rs")).toContain("Arc<ReadonlyRepository<ContactSource>>");
    expect(textOf(rust, "contactSourcesRepository.rs")).not.toContain("Arc<CRUDRepository<ContactSource>>");
    expect(textOf(rust, "contactsRepository.rs")).toContain("Arc<CRUDRepository<Contact>>");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain("RepositoryFactory.bind(register)");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain('export { contact } from "../types/view/contact"');
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("use super::super::Contact");
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("Contact");
    expect(textOf(rust, "repositoryRegistry.rs")).not.toContain("use super::super::;");
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("impl Repositories for Arc<dyn Datasource>");
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("fn contacts(&self) -> Arc<super::contactsRepository::ContactsRepository>");
    expect(textOf(rust, "contactsRepository.rs")).toContain("let mut repo = ReadonlyRepository::<Contact>::new(connection, options)?;");
    expect(textOf(rust, "addressTypesRepository.rs")).toContain("let repo = ReadonlyRepository::<AddressType>::new(connection, options)?;");
    expect(textOf(rust, "addressTypesRepository.rs")).not.toContain("Association");
    expect(textOf(rust, "addressTypesRepository.rs")).not.toContain("add_enriched");
    expect(textOf(rust, "addressTypesRepository.rs")).not.toContain("Enrichment::");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain("readonly contacts: ContactsRepository;");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain('"Contact", "contacts"');
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain('"ContactGroup", "contactGroups"');
    expect(textOf(csharp, "AddressesRepository.cs")).not.toMatch(/ReadProperties = \[[^\]]*AddressTypeName/);
    expect(textOf(csharp, "AddressesRepository.cs")).toMatch(/ReadProperties = \[[^\]]*AddressTypeId/);
    expect(textOf(csharp, "AddressesRepository.cs")).not.toMatch(/CreateProperties = \[[^\]]*AddressTypeId/);
    expect(textOf(csharp, "AddressesRepository.cs")).not.toMatch(/UpdateProperties = \[[^\]]*AddressTypeId/);
    expect(textOf(csharp, "AddressesRepository.cs")).toContain('["AddressTypeName"] = new("string"');
    expect(textOf(csharp, "AddressesRepository.cs")).toContain('AddEnriched(new("AddressTypeName"');
    expect(textOf(typescript, "addressesRepository.ts")).not.toMatch(/readProperties: \[[^\]]*addressTypeName/);
    expect(textOf(typescript, "addressesRepository.ts")).toMatch(/readProperties: \[[^\]]*addressTypeId/);
    expect(textOf(typescript, "addressesRepository.ts")).not.toMatch(/createProperties: \[[^\]]*addressTypeId/);
    expect(textOf(typescript, "addressesRepository.ts")).not.toMatch(/updateProperties: \[[^\]]*addressTypeId/);
    expect(textOf(typescript, "addressesRepository.ts")).toContain('addressTypeName: { type: "string"');
    expect(textOf(typescript, "addressesRepository.ts")).toContain('destinationProperty: "addressTypeName"');
    expect(textOf(rust, "addressesRepository.rs")).not.toMatch(/read_properties = vec!\[[^\]]*address_type_name/);
    expect(textOf(rust, "addressesRepository.rs")).toMatch(/read_properties = vec!\[[^\]]*address_type_id/);
    expect(textOf(rust, "addressesRepository.rs")).not.toMatch(/create_properties = Some\(vec!\[[^\]]*address_type_id/);
    expect(textOf(rust, "addressesRepository.rs")).not.toMatch(/update_properties = Some\(vec!\[[^\]]*address_type_id/);
    expect(textOf(rust, "addressesRepository.rs")).toContain('("address_type_name".into(), SchemaColumn { type_name: "string"');
    expect(textOf(rust, "addressesRepository.rs")).toContain('Enrichment::new("address_type_name"');
    expect(textOf(rust, "phonesRepository.rs")).not.toMatch(/read_properties = vec!\[[^\]]*phone_type_name/);
    expect(textOf(rust, "phonesRepository.rs")).toMatch(/read_properties = vec!\[[^\]]*phone_type_id/);
    expect(textOf(rust, "phonesRepository.rs")).toContain('Enrichment::new("phone_type_name"');
    expect(contactTs).toContain("createReadonly");
    expect(contactTs.slice(contactTs.indexOf("createReadonly"))).not.toContain("ids:");
    expect(legacyTs.slice(legacyTs.indexOf("createReadonly"))).not.toContain("ids:");
    expect(contactTs).toContain("new ReadonlyRepository<Contact>(connection, readonlyOptions(connection))");
    const readonlyFn = contactTs.slice(contactTs.indexOf("const readonlyOptions"), contactTs.indexOf("export class"));
    expect(readonlyFn).toContain("readProperties:");
    expect(readonlyFn).not.toContain("createProperties:");
    expect(readonlyFn).not.toContain("updateProperties:");
    expect(contactTs).not.toMatch(/from "[^"]+\.ts"/);
    expect(legacyTs).not.toMatch(/from "[^"]+\.ts"/);
    expect(textOf(typescript, "repositoryRegistry.ts")).not.toMatch(/from "[^"]+\.ts"/);
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("datasources::bind_repositories!(register);");
    expect(textOf(rust, "repositoryRegistry.rs")).not.toContain("#[ctor::ctor]");
    expect(textOf(rust, "repositoryRegistry.rs")).not.toContain("Once");
    expect(textOf(rust, "repositoryRegistry.rs")).not.toContain("bind();\n");
    expect(textOf(rust, "repositoryRegistry.rs")).toContain('"Contact"');
    expect(textOf(rust, "repositoryRegistry.rs")).toContain('"contacts"');
    expect(textOf(rust, "repositoryRegistry.rs")).toContain('"contact_groups"');
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("fn contacts(&self)");
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("fn contact_groups(&self)");
    expect(textOf(rust, "mod.rs")).toContain("pub mod repositoryRegistry;");
  });

  it("drops remove_fields from create and update on every language", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - template_type:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - template_base:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - template_type_id:
            references: template_type.id
        - name:
            type: string
        - content:
            type: string
  - template:
      tags: [view_type]
      inherits: template_base
      remove_fields:
        - template_type_id
      union:
        - template_type
      extract:
        - template_type.name
      mapping:
        template_type.name: template_type_name
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    };
    const spec = await DeterministicParser(memoryReader(files)).parse({ "backend.datasources": "sqlite" });
    const csharp = planRepositories(spec, "csharp", { "backend.datasources": "sqlite" });
    const typescript = planRepositories(spec, "typescript", {
      "backend.datasources": "sqlite",
      "languages.typescript.casing.fields": "camel",
    });
    const rust = planRepositories(spec, "rust", { "backend.datasources": "sqlite" });
    const template = (models: ReturnType<typeof planRepositories>) => models.find((model) => model.specName === "template");
    expect(template(csharp)?.readProperties).not.toContain("TemplateTypeName");
    expect(template(csharp)?.readProperties).toContain("TemplateTypeId");
    expect(template(csharp)?.createProperties).not.toContain("TemplateTypeId");
    expect(template(csharp)?.updateProperties).not.toContain("TemplateTypeId");
    expect(template(csharp)?.createProperties).not.toContain("TemplateTypeName");
    expect(template(csharp)?.updateProperties).not.toContain("TemplateTypeName");
    expect(template(csharp)?.enrichments.some((item) => item.destination === "TemplateTypeName")).toBe(true);
    expect(template(typescript)?.readProperties).not.toContain("templateTypeName");
    expect(template(typescript)?.readProperties).toContain("templateTypeId");
    expect(template(typescript)?.createProperties).not.toContain("templateTypeId");
    expect(template(typescript)?.updateProperties).not.toContain("templateTypeId");
    expect(template(rust)?.readProperties).not.toContain("template_type_name");
    expect(template(rust)?.readProperties).toContain("template_type_id");
    expect(template(rust)?.createProperties).not.toContain("template_type_id");
    expect(template(rust)?.updateProperties).not.toContain("template_type_id");
    expect(template(rust)?.createProperties).not.toContain("template_type_name");
    expect(template(rust)?.updateProperties).not.toContain("template_type_name");
    const models = planRepositories(
      await DeterministicParser(memoryReader(contactsFiles())).parse({ "backend.datasources": "sqlite" }),
      "csharp",
      { "backend.datasources": "sqlite" },
    );
    const address = models.find((model) => model.specName === "address");
    const contactId = models.find((model) => model.specName === "contact")?.schema.find((column) => column.isId);
    expect(address?.readProperties).not.toContain("AddressTypeName");
    expect(address?.readProperties).toContain("AddressTypeId");
    expect(address?.createProperties).not.toContain("AddressTypeId");
    expect(address?.updateProperties).not.toContain("AddressTypeId");
    expect(address?.schema.some((column) => column.from === "AddressTypeId")).toBe(false);
    expect(address?.propertyMappings).toContainEqual({ from: "AddressTypeId", to: "address_type_id" });
    expect(address?.schema.some((column) => column.from === "AddressTypeName" && column.type === "string")).toBe(true);
    expect(address?.schema.some((column) => column.from === "ContactId" && column.type === contactId?.type)).toBe(true);
    expect(address?.enrichments.some((item) => item.destination === "AddressTypeName" && item.kind === "object")).toBe(true);
  });

  it("plans union enrichments from authored keys, extract filters, and a types-only member", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - source:
      tags: [view_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
        - title:
            type: string
  - nameless:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - code:
            type: string
  - item:
      tags: [datasource_type, view_type]
      union: [source, nameless, missing, code_only]
      extract:
        - source.name
        - nameless.code
        - other.skip
        - title
        - ghost
      mapping:
        name: source_name
      fields:
        - id:
            type: integer
            is_id: true
        - source_id:
            references: source.id
        - nameless_id:
            references: nameless.id
        - code_only_id:
            references: code_only.token
        - title:
            type: string
  - code_only:
      tags: [datasource_type]
      fields:
        - token:
            type: string
            is_id: true
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    };
    const spec = await DeterministicParser(memoryReader(files)).parse({ "backend.datasources": "sqlite" });
    spec.expandedTypes = spec.expandedTypes.filter((type) => type.name !== "source");
    const rust = planRepositories(spec, "rust", { "backend.datasources": "sqlite" });
    const item = rust.find((model) => model.specName === "item");
    expect(item?.enrichments.filter((row) => row.kind === "object").map((row) => row.destination)).toEqual([
      "source_name",
      "title",
      "code",
    ]);
    expect(() =>
      planRepositories(
        { ...spec, expandedTypes: spec.expandedTypes.filter((type) => type.name !== "item") },
        "rust",
        { "backend.datasources": "sqlite" },
      ),
    ).toThrow("item: no expanded type");
  });

  it("keeps default write lists when an engine file omits a type", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - item:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - note:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - body:
            type: string
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
      "datasource.mysql.yaml": `version: 1.0.0
includes:
  - types:
      filter: type == "item"
types: []
`,
    };
    const csharp = textOf(
      await deterministic.generateRepositories({
        dialect: "csharp",
        reader: memoryReader(files),
        settings: { "backend.datasources": "sqlite,mysql" },
      }),
      "NotesRepository.cs",
    );
    expect(csharp).toContain('CreateProperties = ["Body"]');
  });

  it("generates update and delete overrides for optimistic concurrency", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - account:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - version:
            type: integer
        - name:
            type: string
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - account:
      fields:
        - version:
            is_optimistic_concurrency: true
`,
    };
    const occ = (dialect: Dialect) => ({
      dialect,
      reader: memoryReader(files),
      settings: { "backend.datasources": "sqlite" },
    });
    const csharp = textOf(await deterministic.generateRepositories(occ("csharp")), "AccountsRepository.cs");
    const typescript = textOf(await deterministic.generateRepositories(occ("typescript")), "accountsRepository.ts");
    const rust = textOf(await deterministic.generateRepositories(occ("rust")), "accountsRepository.rs");
    const registry = textOf(await deterministic.generateRepositories(occ("rust")), "repositoryRegistry.rs");
    expect(csharp).toContain("public new async Task<Response> Update");
    expect(csharp).toContain("public new async Task<Response> Delete");
    expect(csharp).toContain("item.Version += 1");
    expect(csharp).toContain('.And("Version", eq, Value.From(expected))');
    expect(csharp).toContain("RowsAffected == 1");
    expect(csharp).not.toContain("OptimisticConcurrency");
    expect(typescript).toContain("this.update = async (item, byWhere)");
    expect(typescript).toContain("this.delete = async (item, byWhere)");
    expect(typescript).toContain("item.version = Number(expected) + 1");
    expect(typescript).toContain('.and("version", eq, expected)');
    expect(typescript).toContain("rowsAffected !== 1");
    expect(typescript).not.toContain("optimisticConcurrency:");
    expect(rust).toContain("pub struct AccountOptimistic");
    expect(rust).not.toContain("{{typeName}}Optimistic");
    expect(rust).toContain("item.version += 1");
    expect(rust).toContain("rows_affected != 1");
    expect(rust).not.toContain("optimistic_concurrency");
    expect(registry).toContain("AccountOptimistic::new");

    const stamp = {
      "types.yaml": `version: 1.0.0
types:
  - stamp:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - version:
            type: binary
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - stamp:
      use_optimistic_concurrency: true
      fields:
        - version:
            is_optimistic_concurrency: true
`,
    };
    const skipped = {
      "types.yaml": files["types.yaml"],
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - account:
      use_optimistic_concurrency: false
      fields:
        - version:
            is_optimistic_concurrency: true
`,
    };
    const stampReq = { reader: memoryReader(stamp), settings: { "backend.datasources": "sqlite" } };
    const stampCs = textOf(await deterministic.generateRepositories({ dialect: "csharp", ...stampReq }), "StampsRepository.cs");
    const stampTs = textOf(await deterministic.generateRepositories({ dialect: "typescript", ...stampReq }), "stampsRepository.ts");
    const stampRs = textOf(await deterministic.generateRepositories({ dialect: "rust", ...stampReq }), "stampsRepository.rs");
    const skippedCs = textOf(await deterministic.generateRepositories({ dialect: "csharp", reader: memoryReader(skipped), settings: { "backend.datasources": "sqlite" } }), "AccountsRepository.cs");
    expect(stampCs).toContain("public new async Task<Response> Update");
    expect(stampCs).not.toContain("item.Version += 1");
    expect(stampTs).toContain("this.update = async (item, byWhere)");
    expect(stampTs).toContain("await update(item,");
    expect(stampTs).not.toContain("Number(expected)");
    expect(stampRs).toContain("pub struct StampOptimistic");
    expect(stampRs).not.toContain("item.version += 1");
    expect(skippedCs).not.toContain("public new async Task<Response> Update");
  });

  it("imports kebab type files and omits ids from createReadonly", async () => {
    const typescript = await deterministic.generateRepositories(request("typescript", {
      "backend.datasources": "sqlite",
      "languages.typescript.casing.fields": "camel",
      "languages.typescript.casing.file_names": "kebab",
    }));
    const addressType = textOf(typescript, "address-types-repository.ts");
    expect(addressType).toContain('from "../types/datasource/address-type"');
    expect(addressType).not.toContain("addressType");
    expect(addressType).not.toMatch(/from "[^"]+\.ts"/);
    expect(addressType.slice(addressType.indexOf("createReadonly"))).not.toContain("ids:");
    expect(addressType).toContain("extends ReadonlyRepository<AddressType>");
    expect(addressType).toContain("new DictionaryMapper([])");
    for (const list of addressType.match(/new DictionaryMapper\((\[(?:[^[\]]|\[[^[\]]*\])*\])\)/g) ?? []) {
      const pairs = new Function(`return ${list.slice("new DictionaryMapper(".length, -1)}`)() as [string, string][];
      expect(() => new DictionaryMapper(pairs)).not.toThrow();
    }
  });

  it("uses kebab rust files through #[path] and CLI flags override settings.yaml", async () => {
    const rust = await deterministic.generateRepositories(request("rust", {
      "backend.datasources": "sqlite",
      "languages.rust.casing.file_names": "kebab",
    }));
    expect(textOf(rust, "contacts-repository.rs")).toContain("pub struct ContactsRepository");
    expect(textOf(rust, "contacts-repository.rs")).toContain("pub fn contacts_row_repository");
    expect(textOf(rust, "mod.rs")).toContain('#[path = "contacts-repository.rs"]');
    expect(textOf(rust, "mod.rs")).toContain("pub mod contacts_repository;");
    expect(textOf(rust, "mod.rs")).toContain('#[path = "repository-registry.rs"]');
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n  languages:\n    typescript:\n      casing:\n        file_names: camel\n        fields: camel\n";
    const store = mem(seeded);
    expect(
      await exec(
        ["generate", "repositories", "--dialect", "typescript", "--cwd", cwd, "--file-names", "kebab"],
        deps(store.fs),
        cwd,
      ),
    ).toBe(0);
    expect(store.files[join(cwd, "generated/repositories/contacts-repository.ts")]).toContain("export class ContactsRepository");
    expect(store.files[join(cwd, "generated/repositories/contactsRepository.ts")]).toBeUndefined();
  });

  it("uses one repository name and singular entities", async () => {
    const csharp = await deterministic.generateRepositories(request("csharp"));
    expect(textOf(csharp, "AddressTypesRepository.cs")).toContain("public sealed class AddressTypesRepository");
    expect(textOf(csharp, "AddressTypesRepository.cs")).toContain("ReadonlyRepository<Backend.Types.Datasource.AddressType>");
    expect(textOf(csharp, "AddressTypesRepository.cs")).not.toContain("CRUDRepository<Backend.Types.Datasource.AddressType>");
    expect(textOf(csharp, "AddressTypesRepository.cs")).not.toContain("CreateProperties");
    expect(textOf(csharp, "AddressTypesRepository.cs")).not.toContain("UpdateProperties");
    expect(textOf(csharp, "AddressTypesRepository.cs")).not.toContain("class AddressTypeRepository ");
    expect(textOf(csharp, "AddressesRepository.cs")).toContain("public sealed class AddressesRepository");
    expect(textOf(csharp, "AddressesRepository.cs")).toContain('Entity = "Address"');
    expect(textOf(csharp, "AddressesRepository.cs")).toContain("public AddressTypesRepository AddressType");
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - categories:
      tags: [datasource_type, view_type]
      fields:
        - id:
            type: integer
            is_id: true
  - user_base:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - user:
      tags: [view_type]
      inherits: user_base
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    };
    const named = (
      dialect: Dialect,
      spec: Record<string, string> = files,
      settings: Record<string, string> = {},
    ) => ({
      dialect,
      reader: memoryReader(spec),
      settings: { "backend.datasources": "sqlite", ...settings },
      env: { INSTALLED_PROVIDERS: "sqlite" },
    });
    const dialects = ["csharp", "typescript", "rust"] as const;
    const typeFile = {
      csharp: { category: "Category.cs", userBase: "UserBase.cs", user: "User.cs" },
      typescript: { category: "category.ts", userBase: "userBase.ts", user: "user.ts" },
      rust: { category: "category.rs", userBase: "userBase.rs", user: "user.rs" },
    } as const;
    const repoFile = {
      csharp: { categories: "CategoriesRepository.cs", users: "UsersRepository.cs" },
      typescript: { categories: "categoriesRepository.ts", users: "usersRepository.ts" },
      rust: { categories: "categoriesRepository.rs", users: "usersRepository.rs" },
    } as const;
    const pluralType = {
      csharp: ["Categories.cs", "Users.cs"],
      typescript: ["categories.ts", "users.ts"],
      rust: ["categories.rs", "users.rs"],
    } as const;
    const clash = {
      "types.yaml": `version: 1.0.0
types:
  - user:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - users:
      tags: [view_type]
      inherits: user
      fields:
        - extra:
            type: string
`,
      "datasource.yaml": files["datasource.yaml"],
    };
    for (const dialect of dialects) {
      const types = await deterministic.generateTypes(named(dialect));
      const repos = await deterministic.generateRepositories(named(dialect));
      const sql = (await deterministic.generateDatasource(named(dialect)))
        .map((entry) => entry.contents ?? "")
        .join("\n");
      expect(textOf(types, typeFile[dialect].category)).toContain("Category");
      expect(textOf(types, typeFile[dialect].userBase)).toContain("UserBase");
      expect(textOf(types, typeFile[dialect].user)).toMatch(/User/);
      expect(types.some((entry) => pluralType[dialect].some((name) => entry.filename.endsWith(name)))).toBe(false);
      expect(repos.some((entry) => entry.filename.endsWith(repoFile[dialect].categories))).toBe(true);
      expect(repos.some((entry) => entry.filename.endsWith(repoFile[dialect].users))).toBe(true);
      expect(sql).toMatch(/CREATE TABLE "categories"/);
      expect(sql).toMatch(/CREATE TABLE "user_bases"/);
      if (dialect === "csharp") {
        expect(textOf(repos, repoFile.csharp.categories)).toContain("CRUDRepository<Backend.Types.View.Category>");
        expect(textOf(repos, "RepositoryRegistry.cs")).toContain('"Category", "Categories"');
        expect(textOf(repos, "RepositoryRegistry.cs")).toContain('"User", "Users"');
        expect(textOf(types, "View/User.cs")).toContain("Backend.Types.Datasource.UserBase");
      }
      if (dialect === "typescript") {
        expect(textOf(types, "types/view/user.ts")).toContain("export class User");
        expect(textOf(types, "types/view/user.ts")).not.toContain("extends UserBase");
        expect(textOf(repos, "repositoryRegistry.ts")).toContain('"Category", "categories"');
        expect(textOf(repos, "repositoryRegistry.ts")).toContain('"User", "users"');
      }
      if (dialect === "rust") {
        const crate = repos.find((entry) => entry.filename.endsWith("../mod.rs") || entry.filename === "mod.rs");
        expect(crate?.contents).toContain("types/datasource/userBase.rs");
        expect(crate?.contents).toContain("types/view/user.rs");
        expect(crate?.contents).toContain("pub use userBase::*;");
        expect(crate?.contents).toContain("pub use user::*;");
        expect(textOf(repos, "repositoryRegistry.rs")).toContain("fn categories(");
        expect(textOf(repos, "repositoryRegistry.rs")).toContain("fn users(");
      }
      const clashTypes = await deterministic.generateTypes(named(dialect, clash));
      const clashRepos = await deterministic.generateRepositories(named(dialect, clash));
      expect(clashTypes.some((entry) => entry.filename.endsWith(pluralType[dialect][1]))).toBe(false);
      expect(clashRepos.some((entry) => entry.filename.endsWith(repoFile[dialect].users))).toBe(true);
      if (dialect === "csharp") {
        expect(textOf(clashTypes, "View/User.cs")).toContain("Backend.Types.Datasource.User");
      }
      if (dialect === "typescript") {
        expect(textOf(clashTypes, "types/view/user.ts")).toContain("export class User");
        expect(textOf(clashTypes, "types/view/user.ts")).not.toContain("UserBase");
      }
      if (dialect === "rust") {
        const clashMod = clashRepos.find((entry) => entry.filename.endsWith("../mod.rs") || entry.filename === "mod.rs");
        expect(clashMod?.contents).toContain("types/datasource/user.rs");
        expect(clashMod?.contents).toContain("types/view/user.rs");
        expect(clashMod?.contents).toContain("pub mod datasource_user;");
        expect(clashMod?.contents).toContain("pub mod view_user;");
        expect(clashMod?.contents).toContain("pub mod datasource {");
        expect(clashMod?.contents).toContain("pub use super::datasource_user::*;");
        expect(clashMod?.contents).toContain("pub mod view {");
        expect(clashMod?.contents).toContain("pub use super::view_user::*;");
        expect(clashMod?.contents).not.toContain("pub use user::*;");
      }
    }
    const singularTables = await deterministic.generateRepositories(
      request("csharp", { "backend.datasources": "sqlite", "datasource.pluralize_datatable_names": "false" }),
    );
    expect(textOf(singularTables, "AddressTypesRepository.cs")).toContain("public sealed class AddressTypesRepository");
    expect(textOf(singularTables, "AddressTypesRepository.cs")).toMatch(/\["AddressType"\] = "address_type"/);
  });

  it("keeps inherited view fields on a dual-tagged child", async () => {
    const snake = (name: string): string => name.replaceAll(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
    const sorted = (names: string[]): string[] => [...new Set(names)].sort();
    const rustFields = (text: string): string[] => [...text.matchAll(/^\s+pub (\w+): /gm)].map((match) => match[1]!);
    const tsFields = (text: string): string[] =>
      [...text.matchAll(/^\s+(\w+)[!?]?: /gm)].map((match) => snake(match[1]!));
    const csFields = (text: string): string[] => [...text.matchAll(/(\w+) \{ get;/g)].map((match) => snake(match[1]!));
    const csEffective = (
      entries: { kind: string; filename: string; contents?: string }[],
      filename: string,
    ): string[] => {
      const text = textOf(entries, filename);
      const parent = text.match(/class \w+ : (\S+)/)?.[1];
      const fields = csFields(text);
      if (parent === undefined) return fields;
      const leaf = parent.split(".").at(-1)!;
      const next = parent.includes(".Datasource.") ? `Datasource/${leaf}.cs` : `View/${leaf}.cs`;
      return [...csEffective(entries, next), ...fields];
    };
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - games_base:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - game_settings:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - game_id:
            references: games_base.id
        - name:
            type: string
        - value:
            type: string
  - game_settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: game_settings.name
        - value:
            type: string
            references: game_settings.value
  - categories:
      tags: [datasource_type, view_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
  - games_categories:
      tags: [datasource_type, many_to_many]
      fields:
        - id:
            type: integer
            is_id: true
        - game_id:
            references: games_base.id
        - category_id:
            references: categories.id
  - games:
      tags: [view_type]
      inherits: games_base
      fields:
        - settings:
            type: game_settings_map{}
            references: game_settings.game_id
        - categories:
            type: categories[]
            references: games_categories.game_id
  - template_base:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - resource_base:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
  - puzzle_game:
      tags: [datasource_type, view_type]
      inherits: games
      fields:
        - template_id:
            references: template_base.id
        - image_id:
            references: resource_base.id
            is_nullable: true
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    };
    const named = (dialect: Dialect) => ({
      dialect,
      reader: memoryReader(files),
      settings: { "backend.datasources": "sqlite" },
      env: { INSTALLED_PROVIDERS: "sqlite" },
    });
    const required = ["id", "name", "settings", "categories", "template_id", "image_id"];
    const dialects = ["csharp", "typescript", "rust"] as const;
    const fieldsOf = {
      csharp: (types: { kind: string; filename: string; contents?: string }[]) =>
        sorted(csEffective(types, "View/PuzzleGame.cs")),
      typescript: (types: { kind: string; filename: string; contents?: string }[]) =>
        sorted(tsFields(textOf(types, "types/view/puzzleGame.ts"))),
      rust: (types: { kind: string; filename: string; contents?: string }[]) =>
        sorted(rustFields(textOf(types, "types/view/puzzleGame.rs"))),
    };
    let shared: string[] | undefined;
    for (const dialect of dialects) {
      const types = await deterministic.generateTypes(named(dialect));
      const repos = await deterministic.generateRepositories(named(dialect));
      const fields = fieldsOf[dialect](types);
      expect(fields).toEqual(expect.arrayContaining(required));
      shared = shared ?? fields;
      expect(fields).toEqual(shared);
      if (dialect === "csharp") {
        expect(sorted(csEffective(types, "View/Game.cs"))).toEqual(expect.arrayContaining(["id", "name", "settings", "categories"]));
        expect(textOf(repos, "PuzzleGamesRepository.cs")).toContain("CRUDRepository<Backend.Types.View.PuzzleGame>");
      }
      if (dialect === "typescript") {
        expect(sorted(tsFields(textOf(types, "types/view/game.ts")))).toEqual(
          expect.arrayContaining(["id", "name", "settings", "categories"]),
        );
        expect(textOf(repos, "puzzleGamesRepository.ts")).toContain('from "../types/view/puzzleGame"');
      }
      if (dialect === "rust") {
        expect(sorted(rustFields(textOf(types, "types/view/game.rs")))).toEqual(
          expect.arrayContaining(["id", "name", "settings", "categories"]),
        );
        expect(types.some((entry) => entry.filename.endsWith("types/view/category.rs"))).toBe(false);
        expect(textOf(repos, "puzzleGamesRepository.rs")).toContain("use super::super::PuzzleGame");
        const crate = repos.find((entry) => entry.filename.endsWith("../mod.rs") || entry.filename === "mod.rs");
        expect(crate?.contents).toContain("types/view/puzzleGame.rs");
        expect(crate?.contents).toContain("pub use puzzleGame::*;");
        expect(crate?.contents).toContain("types/datasource/puzzleGame.rs");
        expect(crate?.contents).toContain("pub mod datasource {");
        expect(crate?.contents).toContain("pub use super::datasource_puzzleGame::*;");
      }
    }
  });

  it("cases child fields from settings", async () => {
    const csharp = await deterministic.generateRepositories(request("csharp", {
      "backend.datasources": "sqlite",
      "languages.csharp.casing.fields": "camel",
    }));
    expect(textOf(csharp, "ContactsRepository.cs")).toContain("public AddressesRepository address { get; }");
  });

  it("cases layout directories from flags", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    const store = mem(seeded);
    const io = deps(store.fs);
    expect(
      await exec(
        [
          "generate",
          "all",
          "--dialect",
          "typescript",
          "--cwd",
          cwd,
          "--directories",
          "Pascal",
          "--no-migrations",
          "--no-auto-migrate-up",
        ],
        io,
        cwd,
      ),
    ).toBe(0);
    expect(Object.keys(store.files).some((path) => path.includes("Types/Datasource"))).toBe(true);
    expect(Object.keys(store.files).some((path) => path.includes("Repositories"))).toBe(true);
  });

  it("emits a typescript Contact type and sqlite tables", async () => {
    const types = await deterministic.generateTypes(request("typescript"));
    const text = types.filter((entry) => entry.kind === "content").map((entry) => entry.contents).join("\n");
    expect(text).toContain("export class Contact");
    expect(text).toContain("export const contact = (init: Partial<Contact>): Contact => Object.assign(new Contact(), init);");
    expect(text).not.toContain("Omit<");
    expect(text).toContain("export class AddressType");
    expect(text).toContain("export const addressType = (init: Partial<AddressType>): AddressType => Object.assign(new AddressType(), init);");
    expect(text).toContain("firstName");
    expect(text).toContain("contactSourceName");
    const repos = await deterministic.generateRepositories(request("typescript"));
    const contactsRepo = textOf(repos, "contactsRepository.ts");
    expect(contactsRepo).toContain("import { Contact }");
    expect(contactsRepo).not.toContain("import type { Contact }");
    expect(contactsRepo).toContain("type: Contact");
    const sql = await deterministic.generateDatasource(request("typescript"));
    await deterministic.generateDatasource(request("typescript", {}, { INSTALLED_PROVIDERS: "sqlite" }));
    await deterministic.generateDatasource(
      request("typescript", { "backend.datasources": "sqlite,mysql,postgres" }, { INSTALLED_PROVIDERS: "sqlite" }),
    );
    const script = sql.filter((entry) => entry.kind === "content").map((entry) => entry.contents).join("\n");
    expect(script).toMatch(/CREATE TABLE/);
    expect(script).toContain("contacts");
    expect(script).toContain("OldContactsTbl");
    expect(script).toContain("CntID");
    const installedOnly = await deterministic.generateDatasource(
      request("typescript", { "backend.datasources": "sqlite,mysql,postgres" }, { INSTALLED_PROVIDERS: "sqlite" }),
    );
    expect(installedOnly.some((entry) => entry.filename.includes("sql/sqlite"))).toBe(true);
    expect(installedOnly.some((entry) => entry.filename.includes("sql/mysql"))).toBe(false);
    expect(installedOnly.some((entry) => entry.filename.includes("sql/postgres"))).toBe(false);
    expect(
      await deterministic.generateDatasource({
        dialect: "typescript",
        reader: memoryReader(contactsFiles()),
        settings: { "backend.datasources": "sqlite" },
      }),
    ).toEqual([]);
  });

  it("points generated Rust Facet derives at the datasources re-export", async () => {
    const text = textOf(await deterministic.generateTypes(request("rust")), "contactSource.rs");
    expect(text).toContain("#[derive(Clone, Debug, PartialEq, datasources::Facet)]");
    expect(text).toContain("impl Default for ContactSource");
    expect(text).toContain("#[facet(crate = ::datasources::facet)]");
    expect(text).toContain("datasources::chrono::DateTime<datasources::chrono::Utc>");
    expect(text).not.toMatch(/(^|[^:\w])chrono::/);
    expect(text).not.toMatch(/(^|[^:\w])uuid::/);
  });

  it("owns generated Rust module names without crate::types::generated", async () => {
    const types = await deterministic.generateTypes(request("rust"));
    const contact = textOf(types, "contact.rs");
    expect(textOf(types, "address.rs")).toMatch(/pub contact_id: i32/);
    expect(contact).toContain("Vec<Address>");
    expect(contact).not.toContain("crate::types::generated");
    expect(types.filter((entry) => /pub type (\w+) = \1;/.test(entry.contents ?? "")).length).toBe(0);
    const rust = await deterministic.generateRepositories(request("rust"));
    const root = rust.find((entry) => entry.filename.endsWith("../mod.rs") || entry.filename === "mod.rs");
    expect(root?.contents).toContain("#![allow(dead_code, unused_imports, non_snake_case)]");
    expect(root?.contents).toContain("pub use repositories::repositoryRegistry::Repositories;");
    expect(root?.contents?.split("\n").filter((line) => line === "pub mod legacyContact;").length).toBe(1);
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[join(cwd, ".env")] = "INSTALLED_PROVIDERS=sqlite\n";
    const store = mem(seeded);
    expect(
      await exec(
        ["generate", "repositories", "--dialect", "rust", "--cwd", cwd, "--no-migrations", "--no-auto-migrate-up"],
        deps(store.fs),
        cwd,
      ),
    ).toBe(0);
    expect(store.files[join(cwd, "src/generated.rs")]).toBe(
      `#[path = "../generated/mod.rs"]\nmod generated_mod;\npub use generated_mod::*;\npub use generated_mod::Repositories;\n`,
    );
    expect(store.files[join(cwd, "src/types.rs")]).toBeUndefined();
    expect(store.files[join(cwd, "src/datasources_register.rs")]).toBeUndefined();
  });

  it("infers untyped reference fields from the parent column", async () => {
    const csharp = await deterministic.generateTypes(request("csharp"));
    expect(fieldType(textOf(csharp, "View/Address.cs"), "ContactId")).toBe(fieldType(textOf(csharp, "View/Contact.cs"), "Id"));
    expect(fieldType(textOf(csharp, "Datasource/AddressBase.cs"), "ContactId")).toBe(
      fieldType(textOf(csharp, "Datasource/Base.cs"), "Id"),
    );
    expect(textOf(csharp, "View/Address.cs")).not.toMatch(/string ContactId/);
    const typescript = await deterministic.generateTypes(request("typescript"));
    expect(fieldType(textOf(typescript, "datasource/addressBase.ts"), "contactId")).toBe(
      fieldType(textOf(typescript, "datasource/contactsBase.ts"), "id"),
    );
    expect(fieldType(textOf(typescript, "datasource/addressBase.ts"), "contactId")).toBe(
      fieldType(textOf(typescript, "datasource/addressBase.ts"), "id"),
    );
    expect(textOf(typescript, "view/address.ts")).not.toMatch(/contactId:\s*string/);
    const rust = await deterministic.generateTypes(request("rust"));
    expect(fieldType(textOf(rust, "types/view/address.rs"), "contact_id")).toBe(
      fieldType(textOf(rust, "types/view/contact.rs"), "id"),
    );
    expect(fieldType(textOf(rust, "types/datasource/addressBase.rs"), "contact_id")).toBe(
      fieldType(textOf(rust, "types/datasource/contactsBase.rs"), "id"),
    );
    expect(textOf(rust, "types/view/address.rs")).toMatch(/pub contact_id: i32/);
  });

  it("emits C# auto-properties without a semicolon after the accessor", async () => {
    const contacts = (await deterministic.generateTypes(request("csharp")))
      .filter((entry) => entry.kind === "content")
      .map((entry) => entry.contents)
      .join("\n");
    expect(contacts).toContain("{ get; set; }");
    expect(contacts).not.toContain("{ get; set; };");
    const userRoles = (
      await deterministic.generateTypes({
        dialect: "csharp",
        reader: memoryReader(userRolesFiles()),
        settings: { "backend.datasources": "sqlite" },
      })
    )
      .filter((entry) => entry.kind === "content")
      .map((entry) => entry.contents)
      .join("\n");
    expect(userRoles).toContain("{ get; set; }");
    expect(userRoles).toMatch(/\{ get; set; \}\n/);
    expect(userRoles).not.toContain("{ get; set; };");
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
          type: string
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
      mapping:
        missing: gone
      fields:
      - id:
          type: integer
          is_id: true
      - label:
          type: string
      - items:
          type: pointer[]
          references: pointer.id
      - bare:
          type: pointer[]
      - solos:
          type: solo[]
          references: solo.parent_id
      - missing_map:
          type: ghost{}
      - no_owner:
          type: empty_dict{}
      - keyed:
          type: keyed_dict_map{}
          references: keyed_dict.bag_id
      - named:
          type: named_dict_map{}
          references: named_dict.bag_id
  - empty_dict:
      tags: [view_type]
      inherits: dictionary
      fields:
      - key:
          type: string
      - value:
          type: string
  - keyed_dict:
      tags: [datasource_type]
      inherits: set
      fields:
      - bag_id:
          references: bag.id
      - slot:
          type: string
      - value:
          type: string
  - keyed_dict_map:
      tags: [view_type]
      inherits: dictionary
      fields:
      - key:
          type: string
          references: keyed_dict.slot
      - value:
          type: string
  - named_dict:
      tags: [datasource_type]
      inherits: set
      fields:
      - bag_id:
          references: bag.id
      - attribute_name:
          type: string
      - attribute_value:
          type: string
  - named_dict_map:
      tags: [view_type]
      inherits: dictionary
      fields:
      - key:
          type: string
          references: named_dict.attribute_name
      - value:
          type: string
          references: named_dict.attribute_value
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
        types: contacts.types.map(withoutValue("contact_settings_map")),
        expandedTypes: contacts.expandedTypes.map(withoutValue("contact_settings_map")),
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
    const named = models.find((model) => model.specName === "bag")?.enrichments.find((item) => item.destination === "Named");
    expect(named?.relatedKey).toBe("BagId");
    expect(named?.relatedProperty).toBe("AttributeValue");
    expect(named?.dictionaryKey).toBe("AttributeName");
    expect(named?.via).toBe("named_dict");
    const keyed = models.find((model) => model.specName === "bag")?.enrichments.find((item) => item.destination === "Keyed");
    expect(keyed?.dictionaryKey).toBe("Slot");
    expect(keyed?.relatedProperty).toBe("Value");
    const typescriptPlan = planRepositories(spec, "typescript", {
      "backend.datasources": "sqlite",
      "languages.typescript.casing.fields": "camel",
    });
    const rustPlan = planRepositories(spec, "rust", { "backend.datasources": "sqlite" });
    expect(typescriptPlan.find((model) => model.specName === "bag")?.enrichments.find((item) => item.destination === "named")?.dictionaryKey).toBe(
      "attributeName",
    );
    expect(typescriptPlan.find((model) => model.specName === "bag")?.enrichments.find((item) => item.destination === "keyed")?.dictionaryKey).toBe("slot");
    expect(rustPlan.find((model) => model.specName === "bag")?.enrichments.find((item) => item.destination === "named")?.dictionaryKey).toBe(
      "attribute_name",
    );
    expect(rustPlan.find((model) => model.specName === "bag")?.enrichments.find((item) => item.destination === "keyed")?.dictionaryKey).toBe("slot");
    expect(
      csharpRepositoryGenerators.generate(models, { "backend.datasources": "sqlite" }, spec).some(
        (entry) => entry.kind === "content" && entry.contents?.includes('DictionaryKey: "AttributeName"'),
      ),
    ).toBe(true);
    expect(
      typescriptRepositoryGenerators.generate(typescriptPlan, { "backend.datasources": "sqlite", "languages.typescript.casing.fields": "camel" }, spec).some(
        (entry) => entry.kind === "content" && entry.contents?.includes('dictionaryKey: "attributeName"'),
      ),
    ).toBe(true);
    expect(
      rustRepositoryGenerators.generate(rustPlan, { "backend.datasources": "sqlite" }, spec).some(
        (entry) => entry.kind === "content" && entry.contents?.includes('.with_dictionary_key("attribute_name")'),
      ),
    ).toBe(true);
    const dropKeyRef = (name: string) => (type: (typeof spec.types)[number]) =>
      type.name === name
        ? { ...type, fields: type.fields.map((field) => (field.name === "key" ? { ...field, references: undefined } : field)) }
        : type;
    expect(() =>
      planRepositories(
        {
          ...spec,
          types: spec.types.map(dropKeyRef("named_dict_map")),
          expandedTypes: spec.expandedTypes.map(dropKeyRef("named_dict_map")),
        },
        "csharp",
        { "backend.datasources": "sqlite" },
      ),
    ).toThrow("named_dict_map.key has no references");
    const cycle = {
      "types.yaml": `version: 1.0.0
types:
  - left:
      tags: [datasource_type, view_type]
      fields:
      - id:
          type: integer
          is_id: true
      - right_id:
          references: right.id
      - rights:
          type: right[]
          references: right.left_id
  - right:
      tags: [datasource_type, view_type]
      fields:
      - id:
          type: integer
          is_id: true
      - left_id:
          references: left.id
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
    const mismatch = await DeterministicParser(
      memoryReader({
        "types.yaml": `version: 1.0.0
types:
  - address_base:
      tags: [datasource_type]
      fields:
      - id:
          type: integer
          is_id: true
`,
        "datasource.yaml": `version: 1.0.0
types:
  - addresses_base: {}
`,
      }),
    ).parse({});
    expect(() => planRepositories(mismatch, "csharp", {})).toThrow(/no matching type in types.yaml/);
  });

  it("plans schema column info for string, bool, and decimal without spec defaults", async () => {
    const spec = await DeterministicParser(
      memoryReader({
        "types.yaml": `version: 1.0.0
types:
  - item:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
        - is_starred:
            type: boolean
        - score:
            type: decimal
        - uuid:
            type: uuid
            default_value: NewId
        - note:
            type: string
            default_value: ""
        - latitude:
            type: integer
        - parent_id:
            references: item.id
`,
        "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
      }),
    ).parse({});
    const models = planRepositories(spec, "rust", {});
    const rust = rustRepositoryGenerators.generate(models, {}, spec);
    const typescript = typescriptRepositoryGenerators.generate(models, {}, spec);
    const csharp = csharpRepositoryGenerators.generate(planRepositories(spec, "csharp", {}), {}, spec);
    const rustText = textOf(rust, "itemsRepository.rs");
    const tsText = textOf(typescript, "itemsRepository.ts");
    const csharpText = textOf(csharp, "ItemsRepository.cs");
    expect(rustText).toContain('type_name: "string".into()');
    expect(rustText).toContain('type_name: "boolean".into()');
    expect(rustText).toContain("has_default: true");
    expect(rustText).not.toContain("with_create_zeros");
    expect(tsText).toContain('name: { type: "string", nullable: false, hasDefault: false }');
    expect(tsText).toContain('is_starred: { type: "boolean", nullable: false, hasDefault: false }');
    expect(tsText).toContain('note: { type: "string", nullable: false, hasDefault: true }');
    expect(tsText).toContain('uuid: { type: "uuid", nullable: false, hasDefault: true }');
    expect(tsText).not.toContain("this.createZeros");
    expect(csharpText).toContain("Schema = new");
    expect(csharpText).toContain('["Name"] = new("string"');
    expect(csharpText).toContain('["IsStarred"] = new("boolean"');
    expect(csharpText).toContain('["Score"] = new("decimal"');
  });

  it("writes each generate target", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[join(cwd, ".env")] = "INSTALLED_PROVIDERS=sqlite\n";
    const targets = ["all", "types", "repositories", "datasource"] as const;
    for (const target of targets) {
      const store = mem(seeded);
      const io = deps(store.fs);
      expect(
        await exec(
          ["generate", target, "--dialect", "typescript", "--cwd", cwd, "--no-migrations", "--no-auto-migrate-up"],
          io,
          cwd,
        ),
      ).toBe(0);
      expect(io.logs.length).toBeGreaterThan(0);
    }
    const rustTypes = mem(seeded);
    const rustIo = deps(rustTypes.fs);
    expect(await exec(["generate", "types", "--dialect", "rust", "--cwd", cwd], rustIo, cwd)).toBe(0);
    expect(rustIo.logs.length).toBeGreaterThan(0);
    const absolute = mem({
      [join("/tmp/spec", "types.yaml")]: specFile("types.yaml"),
      [join("/tmp/spec", "base.yaml")]: specFile("base.yaml"),
      [join("/tmp/spec", "lookups.yaml")]: specFile("lookups.yaml"),
      [join("/tmp/spec", "datasource.yaml")]: sqliteDatasourceYaml,
    });
    const io = deps(absolute.fs);
    expect(await exec(["generate", "repositories", "--dialect", "rust", "--input", "/tmp/spec", "--output", "/tmp/out"], io, cwd)).toBe(0);
    expect(absolute.files[join("/tmp/out", "repositories/legacyContactsRepository.rs")]).toContain("CntID");
    const missing = mem();
    expect(await exec(["generate", "types", "--dialect", "csharp", "--cwd", cwd], deps(missing.fs), cwd)).toBe(1);
    expect(await exec(["generate", "types", "--dialect", "typescript", "--provider", "missing", "--cwd", cwd], deps(mem(seeded).fs), cwd)).toBe(1);
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
    expect(await exec(["generate", "repositories", "--dialect", "typescript", "--cwd", cwd], io, cwd)).toBe(0);
    const registry = store.files[join(cwd, "generated/repositories/repositoryRegistry.ts")];
    expect(registry).toContain('"NoteCard", "noteCards"');
    expect(registry).not.toContain('"Note"');
    expect(store.files[join(cwd, "generated/repositories/noteCardsRepository.ts")]).toContain('["NoteCard", "notes"]');
    expect(store.files[join(cwd, "generated/repositories/noteRepository.ts")]).toBeUndefined();
  });

  it("does not emit a repository for a used dictionary view", async () => {
    const spec = await DeterministicParser(
      memoryReader({ "types.yaml": typesYaml, "datasource.yaml": defaultDatasourceYaml }),
    ).parse({ "backend.datasources": "sqlite" });
    const names = planRepositories(spec, "csharp", { "backend.datasources": "sqlite" }).map((model) => model.specName);
    expect(names).not.toContain("contact_preferences_map");
    expect(names).not.toContain("contact_settings_map");
  });

  it("emits a type and repository for an unused dictionary with table references", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - setting:
      tags: [datasource_type]
      fields:
        - name:
            type: string
            is_id: true
        - value:
            type: string
  - settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: setting.name
        - value:
            type: string
            references: setting.value
`,
      "datasource.yaml": defaultDatasourceYaml,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const model = planRepositories(spec, "csharp", settings).find((item) => item.specName === "settings_map");
    expect(model?.typeHome).toBe("view");
    expect(model?.table).toBe("settings");
    expect(model?.ids).toEqual(["Key"]);
    expect(model?.readProperties).toEqual(["Key", "Value"]);
    expect(model?.propertyMappings).toEqual(expect.arrayContaining([{ from: "Key", to: "name" }]));
    const types = await deterministic.generateTypes({ dialect: "csharp", reader: memoryReader(files), settings });
    const repos = await deterministic.generateRepositories({ dialect: "csharp", reader: memoryReader(files), settings });
    expect(textOf(types, "View/SettingsMap.cs")).toContain("class SettingsMap");
    expect(textOf(types, "View/SettingsMap.cs")).toContain("Key");
    expect(textOf(types, "View/SettingsMap.cs")).toContain("Value");
    expect(textOf(repos, "SettingsMapsRepository.cs")).toContain("CRUDRepository<Backend.Types.View.SettingsMap>");
  });

  it("emits a type and no repository for an unused dictionary without key references", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - card_labels:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: string
  - note:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
`,
      "datasource.yaml": defaultDatasourceYaml,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const names = planRepositories(spec, "csharp", settings).map((model) => model.specName);
    expect(names).not.toContain("card_labels");
    const types = await deterministic.generateTypes({ dialect: "csharp", reader: memoryReader(files), settings });
    expect(textOf(types, "View/CardLabel.cs")).toContain("class CardLabel");
    expect(textOf(types, "View/CardLabel.cs")).toContain("Key");
    expect(textOf(types, "View/CardLabel.cs")).toContain("Value");
  });

  it("plans a readonly unused dictionary over matching column names", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - setting:
      tags: [datasource_type]
      fields:
        - key:
            type: string
            is_id: true
        - value:
            type: string
  - settings_map:
      tags: [view_type, readonly_lookup]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: setting.key
            default_value: k
        - value:
            type: string
            references: setting.value
            default_value: v
`,
      "datasource.yaml": defaultDatasourceYaml,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const model = planRepositories(spec, "typescript", settings).find((item) => item.specName === "settings_map");
    expect(model?.readonly).toBe(true);
    expect(model?.optimisticConcurrency).toBeUndefined();
    expect(model?.propertyMappings).toEqual([]);
    expect(model?.schema).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ from: "key", hasDefault: true }),
        expect.objectContaining({ from: "value", hasDefault: true }),
      ]),
    );
  });

  it("skips empty-typed dictionary columns and requires an expanded dictionary type", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - setting:
      tags: [datasource_type]
      fields:
        - key:
            type: string
            is_id: true
        - value:
            type: string
  - settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: setting.key
        - value:
            type: string
            references: setting.value
`,
      "datasource.yaml": defaultDatasourceYaml,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const dict = spec.types.find((type) => type.name === "settings_map");
    const value = dict?.fields.find((field) => field.name === "value");
    if (value === undefined) throw new Error("missing value");
    value.type = "";
    const model = planRepositories(spec, "typescript", settings).find((item) => item.specName === "settings_map");
    expect(model?.schema.map((column) => column.from)).toEqual(["key"]);
    spec.expandedTypes = spec.expandedTypes.filter((type) => type.name !== "settings_map");
    expect(() => planRepositories(spec, "typescript", settings)).toThrow("settings_map: no expanded type");
  });

  it("marks an unused dictionary readonly when the table is a readonly lookup", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - setting:
      tags: [datasource_type, readonly_lookup]
      fields:
        - key:
            type: string
            is_id: true
        - value:
            type: string
  - settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: setting.key
        - value:
            type: string
            references: setting.value
`,
      "datasource.yaml": defaultDatasourceYaml,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    expect(planRepositories(spec, "typescript", settings).find((item) => item.specName === "settings_map")?.readonly).toBe(
      true,
    );
  });

  it("maps unused dictionary value columns through datasource mappings", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - setting:
      tags: [datasource_type]
      fields:
        - key:
            type: string
            is_id: true
        - payload:
            type: string
  - settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: setting.key
        - value:
            type: string
            references: setting.payload
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - setting:
      fields:
        - payload:
            mapping: value_text
`,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const model = planRepositories(spec, "typescript", settings).find((item) => item.specName === "settings_map");
    expect(model?.propertyMappings).toEqual(expect.arrayContaining([{ from: "value", to: "value_text" }]));
  });

  it("plans an unused dictionary value without references and rejects a missing table", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - setting:
      tags: [datasource_type]
      fields:
        - key:
            type: string
            is_id: true
        - value:
            type: string
  - settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: setting.key
        - value:
            type: string
`,
      "datasource.yaml": defaultDatasourceYaml,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const model = planRepositories(spec, "typescript", settings).find((item) => item.specName === "settings_map");
    expect(model?.readProperties).toEqual(["key", "value"]);
    expect(model?.propertyMappings).toEqual([]);
    const key = spec.types.find((type) => type.name === "settings_map")?.fields.find((field) => field.name === "key");
    if (key === undefined) throw new Error("missing key");
    key.references = "missing.key";
    expect(() => planRepositories(spec, "typescript", settings)).toThrow("settings_map: no matching type in types.yaml");
  });

  it("stamps optimistic concurrency on an unused dictionary repository", async () => {
    const files = {
      "types.yaml": `version: 1.0.0
types:
  - setting:
      tags: [datasource_type]
      fields:
        - key:
            type: string
            is_id: true
        - value:
            type: string
        - version:
            type: integer
  - settings_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
            references: setting.key
        - value:
            type: string
            references: setting.value
`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - setting:
      use_optimistic_concurrency: true
      fields:
        - version:
            is_optimistic_concurrency: true
`,
    };
    const settings = { "backend.datasources": "sqlite" };
    const spec = await DeterministicParser(memoryReader(files)).parse(settings);
    const model = planRepositories(spec, "typescript", settings).find((item) => item.specName === "settings_map");
    expect(model?.readonly).toBe(false);
    expect(model?.optimisticConcurrency).toEqual({ property: "version", increment: false });
  });

  it("does not emit a repository for a view_type that does not inherit a base type", async () => {
    const spec = await DeterministicParser(
      memoryReader({
        "types.yaml": `version: 1.0.0
types:
  - locale_preferences:
      tags: [view_type]
      fields:
        - locale:
            type: string
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
        "datasource.yaml": defaultDatasourceYaml,
      }),
    ).parse({ "backend.datasources": "sqlite" });
    const models = planRepositories(spec, "csharp", { "backend.datasources": "sqlite" });
    expect(models.map((model) => model.specName)).not.toContain("locale_preferences");
    expect(models.map((model) => model.specName)).toContain("note_card");
    expect(models.find((model) => model.specName === "note_card")?.readonly).toBe(false);
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
    expect(await exec(["generate", "repositories", "--dialect", "typescript", "--cwd", cwd], io, cwd)).toBe(0);
    const registry = store.files[join(cwd, "generated/repositories/repositoryRegistry.ts")];
    expect(registry).toContain('"Kept", "kepts"');
    expect(registry).not.toContain('"Dropped"');
  });

  it("generate all writes migrations and migrates up", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[join(cwd, ".env")] = "INSTALLED_PROVIDERS=sqlite\nPROVIDER=sqlite\nDATASOURCE_URL=sqlite:///./app.sqlite\n";
    const store = mem(seeded);
    const ran: string[] = [];
    const io = deps(store.fs);
    io.run = async (command, args) => {
      ran.push(`${command} ${args.join(" ")}`);
    };
    const previous = {
      PROVIDER: process.env.PROVIDER,
      DATASOURCE_URL: process.env.DATASOURCE_URL,
      INSTALLED_PROVIDERS: process.env.INSTALLED_PROVIDERS,
    };
    delete process.env.PROVIDER;
    delete process.env.DATASOURCE_URL;
    delete process.env.INSTALLED_PROVIDERS;
    try {
      expect(await exec(["generate", "all", "--dialect", "typescript", "--cwd", cwd], io, cwd)).toBe(0);
      expect(store.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
      expect(Object.keys(store.files).some((path) => path.includes("generated/sql"))).toBe(false);
      expect(ran).toEqual([
        "npx migrate-setup --provider sqlite --connection sqlite:///./app.sqlite",
        "npx migrate-up --provider sqlite --connection sqlite:///./app.sqlite --migrations-root sql",
      ]);
      const dry = mem(seeded);
      const dryRan: string[] = [];
      const dryIo = deps(dry.fs);
      dryIo.run = async (command, args) => {
        dryRan.push(`${command} ${args.join(" ")}`);
      };
      expect(await exec(["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--dry-run"], dryIo, cwd)).toBe(0);
      expect(dry.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toBeUndefined();
      expect(dryIo.logs.some((line) => line.includes("migrate-setup --provider sqlite"))).toBe(true);
      expect(dryIo.logs.some((line) => line.includes("migrate-up --provider sqlite"))).toBe(true);
      expect(dryRan).toEqual([]);
      const skipped = mem(seeded);
      const skipIo = deps(skipped.fs);
      expect(
        await exec(["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--no-migrations", "--no-auto-migrate-up"], skipIo, cwd),
      ).toBe(0);
      expect(skipped.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toBeUndefined();
      expect(Object.keys(skipped.files).some((path) => path.includes("generated/sql") || path.includes("/sql/"))).toBe(false);
      expect(skipIo.logs.some((line) => line.includes("migrate-up"))).toBe(false);
      const memory = mem({
        ...seeded,
        [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\nPROVIDER=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n",
      });
      const memoryIo = deps(memory.fs);
      expect(await exec(["generate", "all", "--dialect", "typescript", "--cwd", cwd], memoryIo, cwd)).toBe(0);
      expect(memoryIo.logs.some((line) => line.includes("in-memory"))).toBe(true);
      const mismatch = mem({
        ...seeded,
        [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\nPROVIDER=postgres\nDATASOURCE_URL=postgres://localhost/db\n",
      });
      expect(await exec(["generate", "all", "--dialect", "typescript", "--cwd", cwd], deps(mismatch.fs), cwd)).toBe(1);
    } finally {
      if (previous.PROVIDER === undefined) delete process.env.PROVIDER;
      else process.env.PROVIDER = previous.PROVIDER;
      if (previous.DATASOURCE_URL === undefined) delete process.env.DATASOURCE_URL;
      else process.env.DATASOURCE_URL = previous.DATASOURCE_URL;
      if (previous.INSTALLED_PROVIDERS === undefined) delete process.env.INSTALLED_PROVIDERS;
      else process.env.INSTALLED_PROVIDERS = previous.INSTALLED_PROVIDERS;
    }
  });

  it("writes migrations create into the migraters layout", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[join(cwd, ".env")] = "INSTALLED_PROVIDERS=sqlite\n";
    const store = mem(seeded);
    const io = deps(store.fs);
    expect(await exec(["migrations", "create", "--cwd", cwd], io, cwd)).toBe(0);
    const up = store.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")];
    expect(up).toMatch(/CREATE TABLE/i);
    const dry = mem(seeded);
    const dryIo = deps(dry.fs);
    expect(await exec(["migrations", "create", "--cwd", cwd, "--dry-run"], dryIo, cwd)).toBe(0);
    expect(dry.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toBeUndefined();
    expect(dryIo.logs.some((line) => line.endsWith("sql/sqlite/migrations/0001_initial_up.sql"))).toBe(true);
    const absolute = mem({
      [join("/tmp/spec", "types.yaml")]: typesYaml,
      [join("/tmp/spec", "datasource.yaml")]: sqliteDatasourceYaml,
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\n",
    });
    expect(
      await exec(
        ["migrations", "create", "--input", "/tmp/spec", "--migrations-root", "/tmp/mig"],
        deps(absolute.fs),
        cwd,
      ),
    ).toBe(0);
    expect(absolute.files[join("/tmp/mig", "sqlite/migrations/0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
  });

  it("removes a stale generated pair and a 10-digit custom copy and keeps a handwritten seed", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const migrations = join(cwd, "sql/sqlite/migrations");
    const stale = join(migrations, "0000000002_custom_002_seed_categories_up.sql");
    const handwritten = join(migrations, "0002_seed_categories_up.sql");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[join(cwd, ".env")] = "INSTALLED_PROVIDERS=sqlite\n";
    seeded[stale] = "INSERT INTO categories VALUES (1);\n";
    seeded[handwritten] = "INSERT INTO categories VALUES (2);\n";
    seeded[join(migrations, "0001_up.sql")] = "CREATE TABLE leftover;\n";
    const store = mem(seeded);
    expect(await exec(["migrations", "create", "--cwd", cwd], deps(store.fs), cwd)).toBe(0);
    expect(store.files[stale]).toBeUndefined();
    expect(store.files[join(migrations, "0001_up.sql")]).toMatch(/CREATE TABLE/i);
    expect(store.files[join(migrations, "0001_initial_up.sql")]).toBeUndefined();
    expect(store.files[handwritten]).toBe("INSERT INTO categories VALUES (2);\n");
  });

  it("keeps 0001_initial_up.sql when version snapshots would emit 0001_up.sql", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const migrations = join(cwd, "sql/sqlite/migrations");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[join(cwd, ".env")] = "INSTALLED_PROVIDERS=sqlite\n";
    seeded[join(input, "version-20261016T000000Z-001", "diff.yaml")] = stringify({
      "types.yaml": { added: true, content: parse(typesYaml) },
    });
    seeded[join(migrations, "0001_initial_up.sql")] = "CREATE TABLE leftover;\n";
    seeded[join(migrations, "0001_initial_down.sql")] = "DROP TABLE leftover;\n";
    const store = mem(seeded);
    expect(await exec(["migrations", "create", "--cwd", cwd], deps(store.fs), cwd)).toBe(0);
    expect(store.files[join(migrations, "0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
    expect(store.files[join(migrations, "0001_initial_down.sql")]).toMatch(/DROP TABLE/i);
    expect(store.files[join(migrations, "0001_up.sql")]).toBeUndefined();
    expect(store.files[join(migrations, "0001_down.sql")]).toBeUndefined();
  });

  it("applies SQL casing flags to migrations and repositories", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const spec = {
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - application:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
        - first_name:
            type: string
`,
      [join(input, "settings.yaml")]: "settings:\n  backend:\n    datasources: sqlite\n",
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\n",
    };
    const generate = mem(spec);
    expect(
      await exec(
        [
          "generate",
          "all",
          "--dialect",
          "typescript",
          "--cwd",
          cwd,
          "--table-names",
          "Pascal",
          "--column-names",
          "Camel",
          "--field-names",
          "Pascal",
          "--object-names",
          "upper",
          "--sql-keywords",
          "lower",
          "--no-auto-migrate-up",
        ],
        deps(generate.fs),
        cwd,
      ),
    ).toBe(0);
    const up = generate.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")];
    expect(up).toMatch(/create table "APPLICATIONS"/);
    expect(up).toMatch(/"firstName"/);
    expect(up).not.toMatch(/CREATE TABLE/);
    const repo = generate.files[join(cwd, "generated/repositories/applicationsRepository.ts")];
    expect(repo).toContain('["Application", "APPLICATIONS"]');
    expect(repo).toContain('["FirstName", "firstName"]');
    const create = mem({
      ...spec,
      [join(input, "settings.yaml")]: `settings:
  backend:
    datasources: sqlite
  datasource:
    casing:
      types: Snake
      fields: Snake
      objects: lower
      keywords: upper
`,
    });
    expect(
      await exec(
        [
          "migrations",
          "create",
          "--cwd",
          cwd,
          "--table-names=Pascal",
          "--column-names=Camel",
          "--object-names=upper",
          "--sql-keywords=lower",
        ],
        deps(create.fs),
        cwd,
      ),
    ).toBe(0);
    expect(create.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toMatch(/create table "APPLICATIONS"/);
  });

  it("pluralizes table names from flags and overrides yaml", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const spec = (yaml: string) => ({
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - application:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
`,
      [join(input, "settings.yaml")]: yaml,
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\n",
    });
    const off = mem(spec("settings:\n  backend:\n    datasources: sqlite\n"));
    expect(
      await exec(
        ["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--no-pluralize-table-names", "--no-auto-migrate-up"],
        deps(off.fs),
        cwd,
      ),
    ).toBe(0);
    expect(off.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toMatch(/CREATE TABLE "application"/);
    expect(off.files[join(cwd, "generated/repositories/applicationsRepository.ts")]).toContain(
      '["Application", "application"]',
    );
    const create = mem(spec("settings:\n  backend:\n    datasources: sqlite\n"));
    expect(await exec(["migrations", "create", "--cwd", cwd, "--no-pluralize-table-names"], deps(create.fs), cwd)).toBe(0);
    expect(create.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toMatch(/CREATE TABLE "application"/);
    const on = mem(
      spec(`settings:
  backend:
    datasources: sqlite
  datasource:
    pluralize_datatable_names: false
`),
    );
    expect(
      await exec(
        ["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--pluralize-table-names", "--no-auto-migrate-up"],
        deps(on.fs),
        cwd,
      ),
    ).toBe(0);
    expect(on.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toMatch(/CREATE TABLE "applications"/);
    expect(on.files[join(cwd, "generated/repositories/applicationsRepository.ts")]).toContain(
      '["Application", "applications"]',
    );
  });

  it("accepts --stored-procedures on generate all and migrations create", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsAuthored()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n";
    seeded[join(cwd, ".env")] = "INSTALLED_PROVIDERS=sqlite\n";
    const generate = mem(seeded);
    expect(
      await exec(
        ["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--stored-procedures", "--no-auto-migrate-up"],
        deps(generate.fs),
        cwd,
      ),
    ).toBe(0);
    expect(generate.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
    expect(Object.keys(generate.files).some((path) => path.includes("stored_procedures"))).toBe(false);
    const create = mem(seeded);
    expect(await exec(["migrations", "create", "--cwd", cwd, "--stored-procedures"], deps(create.fs), cwd)).toBe(0);
    expect(create.files[join(cwd, "sql/sqlite/migrations/0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
    expect(Object.keys(create.files).some((path) => path.includes("stored_procedures"))).toBe(false);
  });

  it("keeps type, repo, bind, and table stems on one owner", async () => {
    const entity = (name: string) => `  - ${name}:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
`;
    const filesOf = (names: string[]) => ({
      "types.yaml": `version: 1.0.0\ntypes:\n${names.map(entity).join("")}`,
      "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
    });
    const named = (
      dialect: Dialect,
      files: Record<string, string>,
      settings: Record<string, string> = {},
    ) => ({
      dialect,
      reader: memoryReader(files),
      settings: { "backend.datasources": "sqlite", ...settings },
      env: { INSTALLED_PROVIDERS: "sqlite" },
    });
    const dialects = ["csharp", "typescript", "rust"] as const;
    const contact = filesOf(["contact"]);
    for (const dialect of dialects) {
      const types = await deterministic.generateTypes(named(dialect, contact));
      const repos = await deterministic.generateRepositories(named(dialect, contact));
      const sql = (await deterministic.generateDatasource(named(dialect, contact)))
        .map((entry) => entry.contents ?? "")
        .join("\n");
      expect(types.some((entry) => /contact\.(ts|cs|rs)$/i.test(entry.filename))).toBe(true);
      expect(repos.some((entry) => /ContactsRepository\.(cs)$/i.test(entry.filename) || /contactsRepository\.(ts|rs)$/.test(entry.filename))).toBe(true);
      expect(repos.some((entry) => /ContactRepository\.(cs)$/i.test(entry.filename) || /contactRepository\.(ts|rs)$/.test(entry.filename))).toBe(false);
      expect(sql).toMatch(/CREATE TABLE "contacts"/);
      if (dialect === "csharp") {
        expect(textOf(repos, "RepositoryRegistry.cs")).toContain('"Contact", "Contacts"');
        expect(textOf(repos, "ContactsRepository.cs")).toContain('["Contact"] = "contacts"');
      }
      if (dialect === "typescript") {
        expect(textOf(repos, "repositoryRegistry.ts")).toContain('"Contact", "contacts"');
        expect(textOf(repos, "contactsRepository.ts")).toContain('["Contact", "contacts"]');
        expect(textOf(repos, "repositoryRegistry.ts")).toContain("export { contact }");
      }
      if (dialect === "rust") {
        expect(textOf(repos, "repositoryRegistry.rs")).toContain("fn contacts(");
        expect(textOf(repos, "contactsRepository.rs")).toContain('"contacts"');
      }
      const singular = await deterministic.generateRepositories(
        named(dialect, contact, { "datasource.pluralize_datatable_names": "false" }),
      );
      const singularSql = (await deterministic.generateDatasource(
        named(dialect, contact, { "datasource.pluralize_datatable_names": "false" }),
      ))
        .map((entry) => entry.contents ?? "")
        .join("\n");
      expect(singularSql).toMatch(/CREATE TABLE "contact"/);
      expect(singular.some((entry) => /ContactsRepository\.(cs)$/i.test(entry.filename) || /contactsRepository\.(ts|rs)$/.test(entry.filename))).toBe(true);
      if (dialect === "csharp") expect(textOf(singular, "RepositoryRegistry.cs")).toContain('"Contact", "Contacts"');
      if (dialect === "typescript") expect(textOf(singular, "repositoryRegistry.ts")).toContain('"Contact", "contacts"');
      if (dialect === "rust") expect(textOf(singular, "repositoryRegistry.rs")).toContain("fn contacts(");
    }

    const already = filesOf(["contacts"]);
    for (const dialect of dialects) {
      const types = await deterministic.generateTypes(named(dialect, already));
      const repos = await deterministic.generateRepositories(named(dialect, already));
      expect(textOf(types, dialect === "csharp" ? "Contact.cs" : dialect === "typescript" ? "contact.ts" : "contact.rs")).toMatch(/Contact/);
      expect(repos.some((entry) => /ContactsRepository\.(cs)$/i.test(entry.filename) || /contactsRepository\.(ts|rs)$/.test(entry.filename))).toBe(true);
      expect(repos.some((entry) => /ContactRepository\.(cs)$/i.test(entry.filename) || /contactRepository\.(ts|rs)$/.test(entry.filename))).toBe(false);
    }

    const irregular = filesOf(["leaf", "status"]);
    const sql = (await deterministic.generateDatasource(named("typescript", irregular)))
      .map((entry) => entry.contents ?? "")
      .join("\n");
    expect(sql).toMatch(/CREATE TABLE "leafs"/);
    expect(sql).toMatch(/CREATE TABLE "statuses"/);
    expect(sql).not.toMatch(/CREATE TABLE "leaves"/);
  });
});
