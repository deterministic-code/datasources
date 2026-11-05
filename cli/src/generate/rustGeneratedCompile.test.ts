import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { tmpdir } from "node:os";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { describe, expect, it } from "vitest";
import { deterministic } from "./DeterministicProvider.ts";
import type { Dialect } from "./GenerateRequest.ts";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(here, "../../..");
const crateRoot = join(tmpdir(), "datasources-rust-generated-compile");
const targetDir = join(crateRoot, "target");
const datasourcesPath = join(repoRoot, "rust/datasources").replaceAll("\\", "/");
const restDir = join(repoRoot, "samples/contacts-crud-enriched-medium-rest-shared/deterministic");
const qsDir = join(repoRoot, "quick_starts/contacts_cli/shared/deterministic");

const yaml = (types: string): Record<string, string> => ({
  "types.yaml": `version: 1.0.0\ntypes:\n${types}`,
  "datasource.yaml": `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type"
types: []
`,
});

const field = (name: string, extra = "type: string"): string => `        - ${name}:\n            ${extra}`;

const entity = (name: string, fields: string, tags = "[datasource_type]"): string =>
  `  - ${name}:\n      tags: ${tags}\n      fields:\n${fields}\n`;

const idName = (name = "name"): string => `${field("id", "type: integer\n            is_id: true")}\n${field(name)}`;

const restFiles = (): Record<string, string> => ({
  "types.yaml": readFileSync(join(restDir, "types.yaml"), "utf8"),
  "datasource.yaml": readFileSync(join(restDir, "datasource.sqlite.yaml"), "utf8"),
  "datasource.sqlite.yaml": readFileSync(join(restDir, "datasource.sqlite.yaml"), "utf8"),
  "datasource.mysql.yaml": readFileSync(join(restDir, "datasource.mysql.yaml"), "utf8"),
  "datasource.postgres.yaml": readFileSync(join(restDir, "datasource.postgres.yaml"), "utf8"),
});

const qsFiles = (): Record<string, string> => ({
  "types.yaml": readFileSync(join(qsDir, "types.yaml"), "utf8"),
  "datasource.yaml": readFileSync(join(qsDir, "datasource.yaml"), "utf8"),
  "settings.yaml": readFileSync(join(qsDir, "settings.yaml"), "utf8"),
});

const requestOf = (files: Record<string, string>, settings: Record<string, string> = {}) => ({
  dialect: "rust" as Dialect,
  reader: memoryReader(files),
  settings: { "backend.datasources": "sqlite", ...settings },
  env: { INSTALLED_PROVIDERS: "sqlite" },
});

const ident = (stem: string): string => (/^[A-Za-z_][A-Za-z0-9_]*$/.test(stem) ? stem : stem.replaceAll("-", "_"));

const typesMod = (filenames: string[]): string => {
  const entries = filenames.map((filename) => {
    const stem = filename.replace(/^.*\//, "").replace(/\.rs$/, "");
    const name = ident(stem);
    return { name, path: `#[path = "${filename}"]\npub mod ${name};\n` };
  });
  return `#![allow(dead_code, unused_imports, non_snake_case)]\n${entries.flatMap((entry) => [entry.path, `pub use ${entry.name}::*;\n`]).join("")}`;
};

const writeCrate = (
  crateDir: string,
  entries: { kind: string; filename: string; contents?: string }[],
  typesOnly: boolean,
): Record<string, string> => {
  mkdirSync(join(crateDir, "src"), { recursive: true });
  writeFileSync(
    join(crateDir, "Cargo.toml"),
    `[package]\nname = "generated_compile"\nversion = "0.0.0"\nedition = "2021"\npublish = false\n\n[dependencies]\ndatasources = { path = "${datasourcesPath}" }\n\n[lints.rust]\nwarnings = "deny"\n`,
  );
  const texts: Record<string, string> = {};
  for (const entry of entries) {
    if (entry.kind !== "content" || entry.contents === undefined) continue;
    texts[entry.filename] = entry.contents;
    const path = join(crateDir, "generated", entry.filename);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, entry.contents);
  }
  if (typesOnly) {
    const typeFiles = Object.keys(texts).filter(
      (name) => name.endsWith(".rs") && name.includes("types/") && !name.includes("/repositories/"),
    );
    writeFileSync(join(crateDir, "generated/mod.rs"), typesMod(typeFiles));
  }
  writeFileSync(
    join(crateDir, "src/lib.rs"),
    `#[path = "../generated/mod.rs"]\nmod generated_mod;\npub use generated_mod::*;\n`,
  );
  return texts;
};

const cargoCheck = (crateDir: string): void => {
  const result = spawnSync("cargo", ["check", "--manifest-path", join(crateDir, "Cargo.toml")], {
    encoding: "utf8",
    env: { ...process.env, CARGO_TARGET_DIR: targetDir },
  });
  if (result.status !== 0) {
    throw new Error(result.stderr || result.stdout || `cargo check exited ${result.status}`);
  }
};

const generate = async (
  files: Record<string, string>,
  settings: Record<string, string> = {},
  typesOnly = false,
): Promise<Record<string, string>> => {
  const crateDir = join(crateRoot, `${process.hrtime.bigint()}`);
  const request = requestOf(files, settings);
  const types = await deterministic.generateTypes(request);
  const repos = typesOnly ? [] : await deterministic.generateRepositories(request);
  const texts = writeCrate(crateDir, [...types, ...repos], typesOnly);
  cargoCheck(crateDir);
  return texts;
};

const textOf = (texts: Record<string, string>, filename: string): string => {
  const key = Object.keys(texts).find((name) => name.endsWith(filename));
  if (key === undefined) throw new Error(`missing ${filename}`);
  return texts[key] ?? "";
};

const account = {
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

describe("generated rust compiles", () => {
  it(
    "types",
    async () => {
      await generate(
        yaml(
          entity(
            "address",
            field("city"),
            "[view_type]",
          ) +
            entity(
              "contact",
              `${idName()}\n${field("created", "type: datetime")}\n${field("guid", "type: uuid")}\n${field("note", "type: string\n            is_nullable: true")}\n${field("addresses", "type: address[]")}`,
              "[view_type]",
            ),
        ),
        {},
        true,
      );
    },
    30_000,
  );

  it("plain", async () => {
    const texts = await generate(yaml(entity("contact", idName())));
    expect(textOf(texts, "contactsRepository.rs")).not.toContain("pub struct ContactsRepository");
    expect(textOf(texts, "repositoryRegistry.rs")).toContain("fn contacts(");
  });

  it("list", async () => {
    const texts = await generate(
      yaml(
        entity(
          "contact",
          `${idName()}\n${field("addresses", "type: address[]\n            references: address.contact_id")}`,
        ) +
          entity(
            "address",
            `${idName("city")}\n${field("contact_id", "references: contact.id")}`,
          ),
      ),
    );
    expect(textOf(texts, "contactsRepository.rs")).toContain("pub struct ContactsRepository");
    expect(textOf(texts, "contactsRepository.rs")).toContain("Ok(Self { repo: Arc::new(repo), address })");
    expect(textOf(texts, "contactsRepository.rs")).toContain("EnrichmentPropertyType::List");
  });

  it("object", async () => {
    const texts = await generate(
      yaml(
        entity("source", idName()) +
          `  - contact:
      tags: [datasource_type, view_type]
      union:
        - source
      extract:
        - source.name
      mapping:
        name: source_name
      fields:
        - id:
            type: integer
            is_id: true
        - source_id:
            references: source.id
        - title:
            type: string
`,
      ),
    );
    expect(textOf(texts, "contactsRepository.rs")).toContain("EnrichmentPropertyType::Object");
  });

  it("dictionary", async () => {
    const texts = await generate(
      yaml(
        entity(
          "contact",
          `${idName()}\n${field("settings", "type: setting_map{}\n            references: setting.contact_id")}`,
        ) +
          entity(
            "setting",
            `${idName("key")}\n${field("contact_id", "references: contact.id")}\n${field("value")}`,
          ) +
          `  - setting_map:
      tags: [view_type]
      inherits: dictionary
      fields:
        - key:
            type: string
        - value:
            type: string
`,
      ),
    );
    expect(textOf(texts, "contactsRepository.rs")).toContain("EnrichmentPropertyType::Dictionary");
  });

  it("linked", async () => {
    const texts = await generate(
      yaml(
        entity("contact", idName()) +
          `  - group:
      tags: [datasource_type, view_type]
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
        - members:
            type: contact[]
            references: group_member.group_id
` +
          `  - group_member:
      tags: [datasource_type, many_to_many]
      fields:
        - id:
            type: integer
            is_id: true
        - contact_id:
            references: contact.id
        - group_id:
            references: group.id
`,
      ),
    );
    expect(textOf(texts, "groupsRepository.rs")).toContain("add_linked");
  });

  it("optimistic increment", async () => {
    const texts = await generate(account);
    expect(textOf(texts, "accountsRepository.rs")).toContain("pub struct AccountOptimistic");
    expect(textOf(texts, "accountsRepository.rs")).toContain("item.version += 1");
  });

  it("optimistic stamp", async () => {
    const texts = await generate(stamp);
    expect(textOf(texts, "stampsRepository.rs")).toContain("pub struct StampOptimistic");
    expect(textOf(texts, "stampsRepository.rs")).not.toContain("item.version += 1");
  });

  it("file names snake", async () => {
    const texts = await generate(yaml(entity("contact", idName())), { "languages.rust.casing.file_names": "snake" });
    expect(texts["repositories/contacts_repository.rs"]).toBeDefined();
    expect(textOf(texts, "mod.rs")).toContain("pub mod contacts_repository;");
  });

  it("file names kebab", async () => {
    const texts = await generate(yaml(entity("contact", idName())), { "languages.rust.casing.file_names": "kebab" });
    expect(texts["repositories/contacts-repository.rs"]).toBeDefined();
    expect(textOf(texts, "repositories/mod.rs")).toContain('#[path = "contacts-repository.rs"]');
    expect(textOf(texts, "repositories/mod.rs")).toContain("pub mod contacts_repository;");
  });

  it("fields snake", async () => {
    const texts = await generate(
      yaml(
        entity(
          "contact",
          `${idName()}\n${field("addresses", "type: address[]\n            references: address.contact_id")}`,
        ) + entity("address", `${idName("city")}\n${field("contact_id", "references: contact.id")}`),
      ),
      { "languages.rust.casing.fields": "snake" },
    );
    expect(textOf(texts, "contactsRepository.rs")).toContain("pub fn contacts_repository");
    expect(textOf(texts, "contactsRepository.rs")).toContain("addresses_repository");
  });

  it("fields camel", async () => {
    const texts = await generate(
      yaml(
        entity(
          "contact",
          `${idName()}\n${field("addresses", "type: address[]\n            references: address.contact_id")}`,
        ) + entity("address", `${idName("city")}\n${field("contact_id", "references: contact.id")}`),
      ),
      { "languages.rust.casing.fields": "camel" },
    );
    expect(textOf(texts, "contactsRepository.rs")).toContain("pub fn contactsRepository");
    expect(textOf(texts, "contactsRepository.rs")).toContain("addressesRowRepository");
  });

  it("pluralizes registry methods and tables", async () => {
    const texts = await generate(
      yaml(
        entity("contact", idName()) +
          entity("address", idName()) +
          entity("category", idName()) +
          entity("person", idName()) +
          entity("notification_type", idName()) +
          entity("phone_label", idName()),
      ),
    );
    const registry = textOf(texts, "repositoryRegistry.rs");
    expect(registry).toContain("fn contacts(");
    expect(registry).toContain("fn addresses(");
    expect(registry).toContain("fn categories(");
    expect(registry).toContain("fn people(");
    expect(registry).toContain("fn notification_types(");
    expect(registry).toContain("fn phone_labels(");
    expect(textOf(texts, "contactsRepository.rs")).toContain('"contacts"');
    expect(textOf(texts, "addressesRepository.rs")).toContain('"addresses"');
    expect(textOf(texts, "categoriesRepository.rs")).toContain('"categories"');
    expect(textOf(texts, "peopleRepository.rs")).toContain('"people"');
    expect(textOf(texts, "notificationTypesRepository.rs")).toContain('"notification_types"');
    expect(textOf(texts, "phoneLabelsRepository.rs")).toContain('"phone_labels"');
  });

  it("keeps registry plurals when table names are not pluralized", async () => {
    const texts = await generate(
      yaml(entity("contact", idName()) + entity("notification_type", idName())),
      { "datasource.pluralize_datatable_names": "false" },
    );
    expect(textOf(texts, "repositoryRegistry.rs")).toContain("fn contacts(");
    expect(textOf(texts, "repositoryRegistry.rs")).toContain("fn notification_types(");
    expect(textOf(texts, "contactsRepository.rs")).toContain('"contact"');
    expect(textOf(texts, "notificationTypesRepository.rs")).toContain('"notification_type"');
  });

  it("REST contacts", async () => {
    const texts = await generate(restFiles(), { "languages.typescript.casing.fields": "camel" });
    expect(textOf(texts, "contactsRepository.rs")).toContain(
      "Ok(Self { repo: Arc::new(repo), address, phone, contact_settings, contact_preferences, contact_source })",
    );
    expect(textOf(texts, "repositoryRegistry.rs")).toContain("fn contacts(");
  });

  it("QS contacts", async () => {
    const texts = await generate(qsFiles());
    expect(textOf(texts, "repositoryRegistry.rs")).toContain("fn contacts(");
    expect(textOf(texts, "contactsRepository.rs")).toMatch(/Ok\(Self \{ repo: Arc::new\(repo\), /);
  });
});
