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

type Address = {
  id: number;
  line1: string;
  line2?: string | null;
  addressTypeName: string;
  country?: string | null;
  latitude?: number | null;
  labelNote?: string | null;
  floor?: number | null;
  score?: string | null;
  isVerified?: boolean | null;
};
type Contact = {
  id: number;
  firstName: string;
  lastName: string;
  addresses: Address[];
  phones: { id: number; number: string }[];
};
type ContactRepo = {
  create: (row: Record<string, string | number | object | null>) => Promise<Contact>;
  update: (row: Record<string, string | number | object | null>) => Promise<unknown>;
  readSingle: (query: object) => Promise<Contact | undefined>;
  delete: (row: Contact) => Promise<unknown>;
};

describe("generate-and-register", () => {
  it("creates a contact through Datasource.contacts", async () => {
    const out = mkdtempSync(join(tmpdir(), "contacts-gen-ts-"));
    const url = `sqlite:///${join(out, "app.sqlite")}`;
    const here = cwd();
    try {
      writeFileSync(join(out, "package.json"), JSON.stringify({ name: "app", private: true, type: "module" }));
      execFileSync(process.execPath, [cli, "setup", "--yes", "--languages", "ts"], { encoding: "utf8", cwd: out });
      execFileSync(process.execPath, [cli, "add", "sqlite", "mysql", "postgres", "--languages", "ts"], { encoding: "utf8", cwd: out });
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
        notes: "poet",
        email: "ada@example.com",
        contactSourceName: "Manual",
        uuid: "00000000-0000-0000-0000-000000000001",
        version: "AA==",
        addresses: [
          { addressTypeName: "Home", line1: "12 Analytical Engine", line2: "Room 12", city: "London", region: "England", postalCode: "SW1A 1AA", country: "GB" },
          { addressTypeName: "Work", line1: "1 Strand", city: "London" },
        ],
        phones: [{ phoneTypeName: "Mobile", number: "555-0100" }],
        settings: { theme: "dark" },
        prefs: { home: { locale: "en-GB", timezone: "Europe/London" } },
      });
      expect(created.firstName).toBe("Ada");
      expect(created.addresses).toHaveLength(2);
      expect(created.addresses[0]?.line2).toBe("Room 12");
      expect(created.addresses[0]?.latitude ?? null).toBeNull();
      const loaded = await ds.contacts.readSingle(new Where(new Operator.Equals("id", created.id)));
      expect(loaded?.firstName).toBe("Ada");
      expect(loaded?.addresses).toHaveLength(2);
      await ds.contacts.update({
        id: created.id,
        firstName: "Ada Byron",
        lastName: created.lastName,
        contactSourceName: "Manual",
        uuid: "00000000-0000-0000-0000-000000000001",
        version: "AA==",
        addresses: [
          { id: created.addresses[0]!.id, addressTypeName: "Work", line1: "12 Analytical Engine", line2: "", city: "Cambridge", region: "", country: "FR" },
          { addressTypeName: "Home", line1: "1 College", city: "Cambridge", country: "GB" },
        ],
        phones: [{ id: created.phones[0]!.id, phoneTypeName: "Mobile", number: "555-0111" }],
        settings: { theme: "sepia" },
        prefs: { home: { locale: "en-GB", timezone: "Europe/London" } },
      });
      const after = await ds.contacts.readSingle(new Where(new Operator.Equals("id", created.id)));
      expect(after?.firstName).toBe("Ada Byron");
      expect(after?.addresses.find((row) => row.line1 === "12 Analytical Engine")?.addressTypeName).toBe("Work");
      const omitted = await ds.contacts.create({
        firstName: "Outlier",
        lastName: "Types",
        contactSourceName: "Manual",
        uuid: "00000000-0000-0000-0000-000000000003",
        version: "AA==",
        addresses: [{ addressTypeName: "Home", line1: "1 Null Lane", city: "London" }],
        phones: [],
        settings: {},
        prefs: {},
      });
      expect(omitted.addresses[0]?.latitude ?? null).toBeNull();
      await ds.contacts.update({
        id: omitted.id,
        firstName: "Outlier",
        lastName: "Types",
        contactSourceName: "Manual",
        uuid: "00000000-0000-0000-0000-000000000003",
        version: "AA==",
        addresses: [{
          id: omitted.addresses[0]!.id,
          addressTypeName: "Home",
          line1: "1 Null Lane",
          city: "London",
          latitude: 51,
          labelNote: "side",
          floor: 2,
          score: "10.5",
          externalId: "550e8400-e29b-41d4-a716-446655440000",
          isVerified: true,
          verifiedAt: "2020-01-02T03:04:05.000Z",
          stamp: "AQ==",
        }],
        phones: [],
        settings: {},
        prefs: {},
      });
      const valued = await ds.contacts.readSingle(new Where(new Operator.Equals("id", omitted.id)));
      expect(valued?.addresses[0]?.latitude).toBe(51);
      expect(valued?.addresses[0]?.labelNote).toBe("side");
      expect(valued?.addresses[0]?.floor).toBe(2);
      expect(valued?.addresses[0]?.score).toBe("10.5");
      expect(valued?.addresses[0]?.isVerified).toBe(true);
      await ds.contacts.update({
        id: omitted.id,
        firstName: "Outlier",
        lastName: "Types",
        contactSourceName: "Manual",
        uuid: "00000000-0000-0000-0000-000000000003",
        version: "AA==",
        addresses: [{
          id: omitted.addresses[0]!.id,
          addressTypeName: "Home",
          line1: "1 Null Lane",
          city: "London",
          latitude: 0,
          labelNote: "",
          floor: 0,
          score: "0",
          isVerified: false,
        }],
        phones: [],
        settings: {},
        prefs: {},
      });
      const zeroed = await ds.contacts.readSingle(new Where(new Operator.Equals("id", omitted.id)));
      expect(zeroed?.addresses[0]?.latitude).toBe(0);
      expect(zeroed?.addresses[0]?.labelNote).toBe("");
      expect(zeroed?.addresses[0]?.isVerified).toBe(false);
      await ds.contacts.delete(after ?? created);
    } finally {
      chdir(here);
      rmSync(out, { recursive: true, force: true });
    }
  }, 180_000);
});
