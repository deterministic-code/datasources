import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
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
import { rustRepositories } from "./RustRepositories.ts";
import { typescriptRepositories } from "./TypeScriptRepositories.ts";
import { pluralCamel, pluralPascal, pluralSnake } from "./RepositoryModel.ts";
import { providerByName } from "./providers.ts";
import { flattenSettings, settingsFrom } from "./settingsFlat.ts";
import { writeEntries } from "./writeEntries.ts";
import { defaultDatasourceYaml } from "./loadSpec.ts";
import { register } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";

register("sqlite", new SqliteDriver());

const specDir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/contacts-crud-enriched-medium-rest-shared/deterministic");
const userRolesDir = join(dirname(fileURLToPath(import.meta.url)), "../../../samples/user-roles-simple-cli-shared/deterministic");
const typesYaml = readFileSync(join(specDir, "types.yaml"), "utf8");
const sqliteDatasourceYaml = readFileSync(join(specDir, "datasource.sqlite.yaml"), "utf8");
const mysqlDatasourceYaml = readFileSync(join(specDir, "datasource.mysql.yaml"), "utf8");
const postgresDatasourceYaml = readFileSync(join(specDir, "datasource.postgres.yaml"), "utf8");
const userRolesFiles = (): Record<string, string> => ({
  "types.yaml": readFileSync(join(userRolesDir, "types.yaml"), "utf8"),
  "datasource.yaml": readFileSync(join(userRolesDir, "datasource.yaml"), "utf8"),
});
const contactsFiles = (): Record<string, string> => ({
  "types.yaml": typesYaml,
  "datasource.yaml": sqliteDatasourceYaml,
  "datasource.sqlite.yaml": sqliteDatasourceYaml,
  "datasource.mysql.yaml": mysqlDatasourceYaml,
  "datasource.postgres.yaml": postgresDatasourceYaml,
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
    const legacyTs = textOf(typescript, "legacyContactRepository.ts");
    const contactTs = textOf(typescript, "contactRepository.ts");
    const legacyRs = textOf(rust, "legacyContactRepository.rs");
    const contactRs = textOf(rust, "contactRepository.rs");
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
    expect(contactCs).toContain('CreateProperties = ["Uuid", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"]');
    expect(contactCs).toContain('CreateProperties = ["ContactSourceId", "FirstName", "LastName", "Email", "Notes"]');
    expect(contactCs).toContain('UpdateProperties = ["Uuid", "Version", "ContactSourceId", "FirstName", "LastName", "Email", "Notes"]');
    expect(contactCs).toContain(".For(connection)");
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
    expect(contactTs).toContain("new CRUDRepositoryBuilder<Contact>()");
    expect(contactTs).toContain('.add("mysql", {');
    expect(contactTs).toContain('.add("postgres", {');
    expect(contactTs).toContain('createProperties: ["uuid", "version", "contactSourceId", "firstName", "lastName", "email", "notes"]');
    expect(contactTs).toContain('createProperties: ["contactSourceId", "firstName", "lastName", "email", "notes"]');
    expect(contactTs).toContain("this.createZeros = {");
    expect(contactTs).toContain("firstName: \"\"");
    expect(contactTs).toContain("lastName: \"\"");
    expect(contactTs).not.toContain("uuid:");
    expect(contactTs).toContain(".for(connection)");
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
    expect(textOf(typescript, "contactGroupRepository.ts")).toContain("addLinked");
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
    expect(contactRs).toContain("with_create_zeros");
    expect(contactRs).toContain("CRUDRepositoryBuilder::new()");
    expect(contactRs).toContain('.add_for("mysql"');
    expect(contactRs).toContain('.add_for("postgres"');
    expect(contactRs).toContain('vec!["contact_source_id".into()');
    expect(contactRs).toContain("for_connection(connection.as_ref())");
    expect(contactRs).toContain("pub struct ContactRepository");
    expect(contactRs).toContain("pub address: Arc<CRUDRepository>");
    expect(contactRs).toContain("pub phone: Arc<CRUDRepository>");
    expect(contactRs).toContain("pub contact_source: Arc<CRUDRepository>");
    expect(contactRs).toContain("address_row_repository");
    expect(contactRs).toContain("Arc::clone(&address)");
    expect(textOf(rust, "schema_objects.rs")).toContain("impl SchemaObject for Contact");
    expect(textOf(rust, "contactRepository.rs")).toContain("Schema::new");
    expect(textOf(rust, "contactRepository.rs")).toContain('"integer"');
    expect(textOf(rust, "contactRepository.rs")).not.toContain("impl Entity");
    expect(textOf(rust, "phoneRepository.rs")).toContain("phone_type_name");
    expect(textOf(rust, "contactRepository.rs")).toContain("contact_source_name");
    expect(textOf(rust, "contactGroupRepository.rs")).toContain("add_linked");
    expect(textOf(rust, "contactGroupRepository.rs")).toContain("pub struct ContactGroupRepository");
    expect(textOf(rust, "mod.rs")).toContain("pub mod contactRepository;");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain("ModuleInitializer");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain("RepositoryFactory.Bind(Register)");
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain('"Contact", "Contacts"');
    expect(textOf(csharp, "RepositoryRegistry.cs")).toContain('"ContactGroup", "ContactGroups"');
    expect(textOf(csharp, "ContactRepository.cs")).toContain("CreateReadonly");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain("RepositoryFactory.bind(register)");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain('export { contact } from "../types/generated/views/contact"');
    expect(textOf(rust, "repositoryRegistry.rs")).toContain("fn contacts(&self) -> Arc<CRUDRepository<Contact>>");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain("readonly contacts: ContactRepository;");
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain('"Contact", "contacts"');
    expect(textOf(typescript, "repositoryRegistry.ts")).toContain('"ContactGroup", "contactGroups"');
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
      "NoteRepository.cs",
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
    const csharp = textOf(await deterministic.generateRepositories(occ("csharp")), "AccountRepository.cs");
    const typescript = textOf(await deterministic.generateRepositories(occ("typescript")), "accountRepository.ts");
    const rust = textOf(await deterministic.generateRepositories(occ("rust")), "accountRepository.rs");
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
    expect(rust).toContain("Value::Int(n) => Value::Int(n + 1)");
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
    const stampCs = textOf(await deterministic.generateRepositories({ dialect: "csharp", ...stampReq }), "StampRepository.cs");
    const stampTs = textOf(await deterministic.generateRepositories({ dialect: "typescript", ...stampReq }), "stampRepository.ts");
    const stampRs = textOf(await deterministic.generateRepositories({ dialect: "rust", ...stampReq }), "stampRepository.rs");
    const skippedCs = textOf(await deterministic.generateRepositories({ dialect: "csharp", reader: memoryReader(skipped), settings: { "backend.datasources": "sqlite" } }), "AccountRepository.cs");
    expect(stampCs).toContain("public new async Task<Response> Update");
    expect(stampCs).not.toContain("item.Version += 1");
    expect(stampTs).toContain("this.update = async (item, byWhere)");
    expect(stampTs).toContain("await update(item,");
    expect(stampTs).not.toContain("Number(expected)");
    expect(stampRs).toContain("pub struct StampOptimistic");
    expect(stampRs).not.toContain("Value::Int(n) => Value::Int(n + 1)");
    expect(skippedCs).not.toContain("public new async Task<Response> Update");
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

  it("uses kebab rust files through #[path] and CLI flags override settings.yaml", async () => {
    const rust = await deterministic.generateRepositories(request("rust", {
      "backend.datasources": "sqlite",
      "languages.rust.casing.file_names": "kebab",
    }));
    expect(textOf(rust, "contact-repository.rs")).toContain("pub fn contact_repository");
    expect(textOf(rust, "mod.rs")).toContain('#[path = "contact-repository.rs"]');
    expect(textOf(rust, "mod.rs")).toContain("pub mod contact_repository;");
    expect(textOf(rust, "mod.rs")).toContain('#[path = "repository-registry.rs"]');
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
    seeded[join(input, "settings.yaml")] = "settings:\n  backend:\n    datasources: sqlite\n  languages:\n    typescript:\n      casing:\n        file_names: camel\n        fields: camel\n";
    const store = mem(seeded);
    expect(
      await exec(
        ["generate", "repositories", "--dialect", "typescript", "--cwd", cwd, "--file-names", "kebab"],
        deps(store.fs),
        cwd,
      ),
    ).toBe(0);
    expect(store.files[join(cwd, "generated/typescript/repositories/contact-repository.ts")]).toContain("export class ContactRepository");
    expect(store.files[join(cwd, "generated/typescript/repositories/contactRepository.ts")]).toBeUndefined();
  });

  it("cases child fields from settings", async () => {
    const csharp = await deterministic.generateRepositories(request("csharp", {
      "backend.datasources": "sqlite",
      "languages.csharp.casing.fields": "camel",
    }));
    expect(textOf(csharp, "ContactRepository.cs")).toContain("public AddressRepository address { get; }");
  });

  it("cases layout directories from flags", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
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
    expect(Object.keys(store.files).some((path) => path.includes("typescript/Types/Generated/Datasource"))).toBe(true);
    expect(Object.keys(store.files).some((path) => path.includes("typescript/Repositories"))).toBe(true);
  });

  it("emits a typescript Contact type and sqlite tables", async () => {
    const types = await deterministic.generateTypes(request("typescript"));
    const text = types.filter((entry) => entry.kind === "content").map((entry) => entry.contents).join("\n");
    expect(text).toContain("export interface Contact");
    expect(text).toContain("export const contact = (init: Partial<Contact>): Contact => ({ ...init } as Contact);");
    expect(text).toContain("firstName");
    expect(text).toContain("contactSourceName");
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
    expect(text).toContain("#[facet(crate = ::datasources::facet)]");
    expect(text).toContain("datasources::chrono::DateTime<datasources::chrono::Utc>");
    expect(text).toContain("datasources::uuid::Uuid::new_v4()");
    expect(text).not.toMatch(/(^|[^:\w])chrono::/);
    expect(text).not.toMatch(/(^|[^:\w])uuid::/);
  });

  it("owns generated Rust module names without crate::types::generated", async () => {
    const types = await deterministic.generateTypes(request("rust"));
    const contact = textOf(types, "contact.rs");
    expect(contact).toContain("Vec<Address>");
    expect(contact).not.toContain("crate::types::generated");
    const rust = await deterministic.generateRepositories(request("rust"));
    const root = rust.find((entry) => entry.filename.endsWith("../mod.rs") || entry.filename === "rust/mod.rs");
    expect(root?.contents).toContain("pub use repositories::repositoryRegistry::Repositories;");
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
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
      `#[path = "../generated/rust/mod.rs"]\nmod generated_mod;\npub use generated_mod::*;\n`,
    );
    expect(store.files[join(cwd, "src/types.rs")]).toBeUndefined();
    expect(store.files[join(cwd, "src/datasources_register.rs")]).toBeUndefined();
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
    expect(userRoles).toMatch(/\{ get; set; \} = /);
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
      mapping:
        missing: gone
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
      - key:
          type: string
      - value:
          type: string
  - keyed_dict_map:
      tags: [view_type]
      inherits: dictionary
      fields:
      - key:
          type: string
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
    expect(named?.via).toBe("named_dict");
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

  it("plans createZeros for string and bool without spec defaults", async () => {
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
    expect(models.find((model) => model.specName === "item")?.createZeros).toEqual([
      { name: "name", kind: "string" },
      { name: "is_starred", kind: "boolean" },
    ]);
    const rust = rustRepositories(models, {});
    const typescript = typescriptRepositories(models, {});
    expect(textOf(rust, "itemRepository.rs")).toContain("Value::from(false)");
    expect(textOf(rust, "itemRepository.rs")).not.toContain("Value::from(0)");
    expect(textOf(typescript, "itemRepository.ts")).toContain("is_starred: false");
  });

  it("writes each generate target", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
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
      [join("/tmp/spec", "types.yaml")]: typesYaml,
      [join("/tmp/spec", "datasource.yaml")]: sqliteDatasourceYaml,
    });
    const io = deps(absolute.fs);
    expect(await exec(["generate", "repositories", "--dialect", "rust", "--input", "/tmp/spec", "--output", "/tmp/out"], io, cwd)).toBe(0);
    expect(absolute.files[join("/tmp/out", "rust/repositories/legacyContactRepository.rs")]).toContain("CntID");
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
    const registry = store.files[join(cwd, "generated/typescript/repositories/repositoryRegistry.ts")];
    expect(registry).toContain('"NoteCard", "noteCards"');
    expect(registry).not.toContain('"Note"');
    expect(store.files[join(cwd, "generated/typescript/repositories/noteCardRepository.ts")]).toContain('["NoteCard", "notes"]');
    expect(store.files[join(cwd, "generated/typescript/repositories/noteRepository.ts")]).toBeUndefined();
  });

  it("does not emit a repository for a dictionary view", async () => {
    const spec = await DeterministicParser(
      memoryReader({ "types.yaml": typesYaml, "datasource.yaml": defaultDatasourceYaml }),
    ).parse({ "backend.datasources": "sqlite" });
    const names = planRepositories(spec, "csharp", { "backend.datasources": "sqlite" }).map((model) => model.specName);
    expect(names).not.toContain("card_labels");
    expect(names).not.toContain("contact_preferences_map");
    expect(names).not.toContain("contact_settings_map");
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
    const registry = store.files[join(cwd, "generated/typescript/repositories/repositoryRegistry.ts")];
    expect(registry).toContain('"Kept", "kepts"');
    expect(registry).not.toContain('"Dropped"');
  });

  it("generate all writes migrations and migrates up", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
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
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
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
    const repo = generate.files[join(cwd, "generated/typescript/repositories/applicationRepository.ts")];
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
    expect(off.files[join(cwd, "generated/typescript/repositories/applicationRepository.ts")]).toContain(
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
    expect(on.files[join(cwd, "generated/typescript/repositories/applicationRepository.ts")]).toContain(
      '["Application", "applications"]',
    );
  });

  it("accepts --stored-procedures on generate all and migrations create", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const seeded = Object.fromEntries(Object.entries(contactsFiles()).map(([name, text]) => [join(input, name), text]));
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
});
