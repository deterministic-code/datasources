import { Datasource, Native, Operator, Where, type RepositoryFactory } from "@datasources/core";
import { SqliteDriver } from "@datasources/sqlite";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { describe, expect, it } from "vitest";
import { statements } from "../src/schema.ts";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
const spec = resolve(sample, "../../contacts-crud-enriched-medium-rest-shared/deterministic");

type ContactRepo = {
  create: (row: Record<string, string | number | object>) => Promise<{ id: number; firstName: string }>;
  readSingle: (query: Where) => Promise<{ firstName: string } | undefined>;
};

describe("generate-and-register", () => {
  it("creates a contact through Datasource.contacts", async () => {
    const out = mkdtempSync(join(tmpdir(), "contacts-gen-ts-"));
    try {
      execFileSync(
        process.execPath,
        [
          "--experimental-strip-types",
          join(repo, "cli/src/main.ts"),
          "generate",
          "all",
          "--dialect",
          "typescript",
          "--input",
          spec,
          "--output",
          out,
        ],
        { encoding: "utf8", cwd: join(repo, "cli") },
      );
      const { register } = (await import(
        pathToFileURL(join(out, "typescript/repositories/repository-registry.ts")).href
      )) as { register: (factory: RepositoryFactory) => void };
      const ds = new Datasource("sqlite", new SqliteDriver(), {
        DATASOURCE_URL: "sqlite:///:memory:",
      }) as Datasource & { contacts: { create: () => ContactRepo } };
      register(ds.repositories);
      const command = await ds.createCommand();
      await command.executeNative(new Native("PRAGMA foreign_keys = ON"));
      for (const sql of statements(readFileSync(join(out, "sql/sqlite/migrations/0001_initial_up.sql"), "utf8")))
        await command.executeNative(new Native(sql));
      const contacts = ds.contacts.create();
      const created = await contacts.create({
        firstName: "Ada",
        lastName: "Lovelace",
        contactSourceId: 1,
        uuid: "00000000-0000-0000-0000-000000000001",
        version: "AA==",
        addresses: [],
        phones: [],
        settings: {},
        prefs: {},
      });
      expect(created.firstName).toBe("Ada");
      const loaded = await contacts.readSingle(new Where(new Operator.Equals("id", created.id)));
      expect(loaded?.firstName).toBe("Ada");
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });
});
