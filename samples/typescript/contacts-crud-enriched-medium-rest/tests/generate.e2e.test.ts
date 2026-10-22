import { Datasource, Operator, Where } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { describe, expect, it } from "vitest";
import { migrateUp } from "../../../../scripts/migraters.mjs";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
const spec = resolve(sample, "../../contacts-crud-enriched-medium-rest-shared/deterministic");

type Contact = { id: number; firstName: string };
type ContactRepo = {
  create: (row: Record<string, string | number | object>) => Promise<Contact>;
  readSingle: (query: Where) => Promise<Contact | undefined>;
};

describe("generate-and-register", () => {
  it("creates a contact through Datasource.contacts", async () => {
    const out = mkdtempSync(join(tmpdir(), "contacts-gen-ts-"));
    const url = `sqlite:///${join(out, "app.sqlite")}`;
    try {
      execFileSync(
        process.execPath,
        [
          "--import",
          "tsx",
          join(repo, "cli/src/main.ts"),
          "generate",
          "all",
          "--dialect",
          "typescript",
          "--input",
          spec,
          "--output",
          "generated",
          "--cwd",
          out,
          "--no-migrate",
        ],
        {
          encoding: "utf8",
          cwd: join(repo, "cli"),
          env: { ...process.env, PROVIDER: "sqlite", DATASOURCE_URL: url },
        },
      );
      migrateUp({ provider: "sqlite", connection: url, migrationsRoot: join(out, "sql"), cwd: sample });
      await import(pathToFileURL(join(out, "generated/typescript/repositories/repository-registry.ts")).href);
      const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: url }) as Datasource & {
        contacts: ContactRepo;
      };
      await (await ds.createConnection()).open();
      const created: Contact = await ds.contacts.create({
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
      const loaded: Contact | undefined = await ds.contacts.readSingle(new Where(new Operator.Equals("id", created.id)));
      expect(loaded?.firstName).toBe("Ada");
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  }, 180_000);
});
