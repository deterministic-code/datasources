import { execFileSync } from "node:child_process";
import { createRequire } from "node:module";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { chdir, cwd } from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";
import { describe, expect, it } from "vitest";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
const spec = resolve(sample, "../../contacts-crud-enriched-medium-rest-shared/deterministic");
const cli = join(repo, "cli/bin/datasources.mjs");

type Contact = { id: number; firstName: string };
type ContactRepo = {
  create: (row: Record<string, string | number | object>) => Promise<Contact>;
  readSingle: (query: object) => Promise<Contact | undefined>;
};

describe("generate-and-register", () => {
  it("creates a contact through Datasource.contacts", async () => {
    const out = mkdtempSync(join(tmpdir(), "contacts-gen-ts-"));
    const url = `sqlite:///${join(out, "app.sqlite")}`;
    const here = cwd();
    try {
      writeFileSync(join(out, "package.json"), JSON.stringify({ name: "app", private: true, type: "module" }));
      execFileSync(process.execPath, [cli, "setup", "--yes", "--ecosystem", "ts"], { encoding: "utf8", cwd: out });
      execFileSync(process.execPath, [cli, "add", "sqlite", "mysql", "postgres", "--ecosystem", "ts"], { encoding: "utf8", cwd: out });
      execFileSync(process.execPath, [cli, "generate", "all", "--dialect", "typescript", "--input", spec, "--output", "generated", "--no-auto-migrate-up"], {
        encoding: "utf8",
        cwd: out,
        env: { ...process.env, PROVIDER: "sqlite", DATASOURCE_URL: url },
      });
      const db = join(out, "app.sqlite");
      execFileSync("npx", ["migrate-setup", "--provider", "sqlite", "--connection", db], { encoding: "utf8", cwd: out });
      execFileSync(
        "npx",
        ["migrate-up", "--provider", "sqlite", "--connection", db, "--migrations-root", "sql"],
        { encoding: "utf8", cwd: out },
      );
      chdir(out);
      await import(pathToFileURL(join(out, "datasources-register.ts")).href);
      const { Datasource, Operator, Where } = await import(
        pathToFileURL(createRequire(join(out, "package.json")).resolve("@deterministic-code/datasources")).href
      );
      process.env.PROVIDER = "sqlite";
      process.env.DATASOURCE_URL = url;
      const ds = Datasource.default as { contacts: ContactRepo; createConnection: () => Promise<{ open: () => Promise<void> }> };
      await (await ds.createConnection()).open();
      const created = await ds.contacts.create({
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
      const loaded = await ds.contacts.readSingle(new Where(new Operator.Equals("id", created.id)));
      expect(loaded?.firstName).toBe("Ada");
    } finally {
      chdir(here);
      rmSync(out, { recursive: true, force: true });
    }
  }, 180_000);
});
