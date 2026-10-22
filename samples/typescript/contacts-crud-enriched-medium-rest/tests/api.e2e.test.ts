import { ConsoleCommandMiddleware, ExpectConsoleMiddleware, type ICommandMiddleware } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import { Api } from "../src/api.ts";
import { clock } from "../src/clock.ts";
import { LegacyContactRepository } from "../generated/typescript/repositories/legacy-contact-repository.ts";
import { ContactRepository } from "../generated/typescript/repositories/contact-repository.ts";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const shared = resolve(sample, "../../contacts-crud-enriched-medium-rest-shared");
const logFile = join(shared, "sqlite.log");

const open = async (middleware?: ICommandMiddleware) => {
  const dir = mkdtempSync(join(tmpdir(), "contacts-api-"));
  const db = join(dir, "app.sqlite");
  const url = `sqlite:///${db}`;
  execFileSync("npx", ["migrate-setup", "--provider", "sqlite", "--connection", db], { encoding: "utf8", cwd: sample });
  execFileSync(
    "npx",
    ["migrate-up", "--provider", "sqlite", "--connection", db, "--migrations-root", "sql"],
    { encoding: "utf8", cwd: sample },
  );
  const connection = new SqliteDriver().connection({ DATASOURCE_URL: url });
  await connection.open();
  const contacts = new ContactRepository(connection);
  const legacy = new LegacyContactRepository(connection);
  if (middleware) {
    contacts.addMiddleware(middleware);
    contacts.address.addMiddleware(middleware);
    contacts.phone.addMiddleware(middleware);
    contacts.contactSource.addMiddleware(middleware);
    contacts.contactSettings.addMiddleware(middleware);
    contacts.contactPreferences.addMiddleware(middleware);
    contacts.address.addressType.addMiddleware(middleware);
    contacts.phone.phoneType.addMiddleware(middleware);
    legacy.addMiddleware(middleware);
  }
  return { api: new Api(contacts, legacy), dir };
};

describe("api e2e", () => {
  const now = clock.now;
  const uuid = clock.uuid;
  afterEach(() => {
    clock.now = now;
    clock.uuid = uuid;
  });

  it("round-trips contacts and legacy contacts", async () => {
    const stamp = new Date("2026-09-22T00:00:00.000Z");
    let n = 0;
    const lines: string[] = [];
    const record = process.env.RECORD_CONSOLE === "1";
    const signature = (line: string): string => {
      const duration = line.lastIndexOf("] ");
      const stripped = duration >= 0 && (line.endsWith("ms") || line.endsWith("μs")) ? line.slice(0, duration + 1) : line;
      return stripped
        .replace(/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z/g, "<date>")
        .replace(/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}/g, "<uuid>")
        .replace(/(\d+)\.0(?=[,}\]])/g, "$1");
    };
    const middleware = record
      ? new ConsoleCommandMiddleware((line) => lines.push(signature(line)), () => stamp, { isVerbose: "true" })
      : new ExpectConsoleMiddleware(
          readFileSync(logFile, "utf8").split("\n").filter((line) => line.length > 0),
          () => stamp,
          signature,
        );
    clock.now = () => stamp;
    clock.uuid = () => `00000000-0000-0000-0000-${String(++n).padStart(12, "0")}`;
    const { api, dir } = await open(middleware);
    try {
      const created = await api.handle("POST", "/contacts/", JSON.stringify({
        firstName: "Ada",
        lastName: "Lovelace",
        email: "ada@example.com",
        contactSourceName: "Manual",
        addresses: [
          { line1: "12 Analytical Engine", city: "London", addressTypeName: "Home" },
          { line1: "1 Strand", city: "London", addressTypeName: "Home" },
        ],
        phones: [
          { number: "555-0100", label: "Mobile", phoneTypeName: "Mobile" },
          { number: "555-0199", label: "Work", phoneTypeName: "Mobile" },
        ],
        settings: { theme: "dark", density: "comfortable" },
        prefs: {
          home: { locale: "en-GB", timezone: "Europe/London" },
          travel: { locale: "en-US", timezone: "America/New_York" },
        },
      }));
      expect(created.status).toBe(201);
      type Address = { id: number; line1: string; city: string; addressTypeName: string };
      type Phone = { id: number; number: string; label?: string; phoneTypeName: string };
      type Contact = {
        id: number;
        firstName: string;
        lastName: string;
        contactSourceName: string;
        addresses: Address[];
        phones: Phone[];
        settings: { theme?: string; density?: string; locale?: string; imported?: string; source?: string };
        prefs: { home?: { locale: string; timezone: string }; travel?: { locale: string; timezone: string }; work?: { locale: string; timezone: string } };
      };
      const saved = JSON.parse(created.body) as Contact;
      expect(saved.firstName).toBe("Ada");
      await api.handle("POST", "/contacts", JSON.stringify({
        firstName: "Grace",
        lastName: "Hopper",
        uuid: "22222222-2222-2222-2222-222222222222",
        created: "2024-01-02T00:00:00Z",
        contactSourceName: "Imported",
        addresses: [
          { uuid: "33333333-3333-3333-3333-333333333333", created: "2024-01-02T00:00:00Z", line1: "1", city: "NY", addressTypeName: "Home" },
          { uuid: "55555555-5555-5555-5555-555555555555", created: "2024-01-02T00:00:00Z", line1: "2", city: "Boston", addressTypeName: "Home" },
        ],
        phones: [
          { uuid: "44444444-4444-4444-4444-444444444444", created: "2024-01-02T00:00:00Z", number: "1", phoneTypeName: "Mobile" },
          { uuid: "66666666-6666-6666-6666-666666666666", created: "2024-01-02T00:00:00Z", number: "2", phoneTypeName: "Mobile" },
        ],
        settings: { imported: "yes", source: "csv" },
      }));
      const listed = JSON.parse((await api.handle("GET", "/contacts/", undefined)).body) as Contact[];
      expect(listed).toHaveLength(2);
      const updated = await api.handle("PUT", `/contacts/?id=${saved.id}`, JSON.stringify({
        firstName: "Ada",
        lastName: "Byron",
        email: null,
        contactSourceName: "Manual",
        addresses: [
          { id: saved.addresses.find((row) => row.line1 === "12 Analytical Engine")!.id, line1: "12 Analytical Engine", city: "Cambridge", addressTypeName: "Home" },
          { line1: "10 Downing", city: "London", addressTypeName: "Home" },
        ],
        phones: [
          { id: saved.phones.find((row) => row.number === "555-0100")!.id, number: "555-0101", label: "Mobile", phoneTypeName: "Mobile" },
          { number: "555-0200", label: "Office", phoneTypeName: "Mobile" },
        ],
        settings: { theme: "light", locale: "en" },
        prefs: {
          home: { locale: "en-GB", timezone: "Europe/Paris" },
          work: { locale: "en-US", timezone: "America/New_York" },
        },
      }));
      expect(updated.status).toBe(200);
      expect((await api.handle("DELETE", `/contacts/?id=${saved.id}`, undefined)).status).toBe(204);
      expect((await api.handle("POST", "/legacy_contacts/", JSON.stringify({
        key: "ada",
        firstName: "Ada",
        lastName: "Lovelace",
        email: "ada@example.com",
        importedAt: "2024-06-01T00:00:00Z",
      }))).status).toBe(201);
      expect((await api.handle("DELETE", "/legacy_contacts/?key=ada", undefined)).status).toBe(204);
      if (record) {
        mkdirSync(shared, { recursive: true });
        writeFileSync(logFile, `${lines.join("\n")}\n`);
      } else (middleware as ExpectConsoleMiddleware).complete();
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }, 180_000);

  it("creates a seeded contact", async () => {
    const { api, dir } = await open();
    try {
      expect((await api.handle("POST", "/contacts/", JSON.stringify({ firstName: "Ada", lastName: "Lovelace", contactSourceName: "Manual" }))).status).toBe(201);
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }, 180_000);
});
