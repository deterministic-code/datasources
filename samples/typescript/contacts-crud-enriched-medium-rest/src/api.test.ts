import { ConsoleCommandMiddleware, DatasourceError, ExpectConsoleMiddleware, type ICommandMiddleware } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import { Api } from "./api.ts";
import { clock } from "./clock.ts";
import { LegacyContactRepository } from "../generated/typescript/repositories/legacy-contact-repository.ts";
import { migrate } from "./migrate.ts";
import { ContactRepository } from "../generated/typescript/repositories/contact-repository.ts";

const shared = resolve(dirname(fileURLToPath(import.meta.url)), "../../../contacts-crud-enriched-medium-rest-shared");
const logFile = join(shared, "sqlite.log");

const open = async (middleware?: ICommandMiddleware) => {
  const connection = new SqliteDriver().connection({ DATASOURCE_URL: "sqlite:///:memory:" });
  await connection.open();
  await migrate(connection.createCommand());
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
  return new Api(contacts, legacy);
};

describe("api", () => {
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
    const api = await open(middleware);
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
    expect(created.body).not.toContain("OldContactsTbl");
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
    expect(saved.contactSourceName).toBe("Manual");
    expect(saved.addresses).toHaveLength(2);
    expect(saved.addresses.find((row) => row.line1 === "12 Analytical Engine")).toMatchObject({ city: "London", addressTypeName: "Home" });
    expect(saved.addresses.find((row) => row.line1 === "1 Strand")).toMatchObject({ city: "London", addressTypeName: "Home" });
    expect(saved.phones).toHaveLength(2);
    expect(saved.phones.find((row) => row.number === "555-0100")).toMatchObject({ label: "Mobile", phoneTypeName: "Mobile" });
    expect(saved.phones.find((row) => row.number === "555-0199")).toMatchObject({ label: "Work", phoneTypeName: "Mobile" });
    expect(saved.settings).toEqual({ theme: "dark", density: "comfortable" });
    expect(saved.prefs.home).toEqual({ locale: "en-GB", timezone: "Europe/London" });
    expect(saved.prefs.travel).toEqual({ locale: "en-US", timezone: "America/New_York" });
    const homeAddressId = saved.addresses.find((row) => row.line1 === "12 Analytical Engine")!.id;
    const mobilePhoneId = saved.phones.find((row) => row.number === "555-0100")!.id;
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
    const ada = listed.find((row) => row.firstName === "Ada");
    const grace = listed.find((row) => row.firstName === "Grace");
    expect(ada?.addresses).toHaveLength(2);
    expect(ada?.phones).toHaveLength(2);
    expect(ada?.settings).toEqual({ theme: "dark", density: "comfortable" });
    expect(grace?.addresses).toHaveLength(2);
    expect(grace?.phones).toHaveLength(2);
    expect(grace?.settings).toEqual({ imported: "yes", source: "csv" });
    const updated = await api.handle("PUT", `/contacts/?id=${saved.id}`, JSON.stringify({
      firstName: "Ada",
      lastName: "Byron",
      email: null,
      contactSourceName: "Manual",
      addresses: [
        { id: homeAddressId, line1: "12 Analytical Engine", city: "Cambridge", addressTypeName: "Home" },
        { line1: "10 Downing", city: "London", addressTypeName: "Home" },
      ],
      phones: [
        { id: mobilePhoneId, number: "555-0101", label: "Mobile", phoneTypeName: "Mobile" },
        { number: "555-0200", label: "Office", phoneTypeName: "Mobile" },
      ],
      settings: { theme: "light", locale: "en" },
      prefs: {
        home: { locale: "en-GB", timezone: "Europe/Paris" },
        work: { locale: "en-US", timezone: "America/New_York" },
      },
    }));
    expect(updated.status).toBe(200);
    const changed = JSON.parse(updated.body) as Contact;
    expect(changed.lastName).toBe("Byron");
    expect(changed.addresses).toHaveLength(2);
    expect(changed.addresses.find((row) => row.id === homeAddressId)).toMatchObject({ line1: "12 Analytical Engine", city: "Cambridge" });
    expect(changed.addresses.find((row) => row.line1 === "10 Downing")).toMatchObject({ city: "London" });
    expect(changed.addresses.some((row) => row.line1 === "1 Strand")).toBe(false);
    expect(changed.phones).toHaveLength(2);
    expect(changed.phones.find((row) => row.id === mobilePhoneId)).toMatchObject({ number: "555-0101" });
    expect(changed.phones.find((row) => row.number === "555-0200")).toMatchObject({ label: "Office" });
    expect(changed.phones.some((row) => row.number === "555-0199")).toBe(false);
    expect(changed.settings).toEqual({ theme: "light", locale: "en" });
    expect(changed.prefs.home).toEqual({ locale: "en-GB", timezone: "Europe/Paris" });
    expect(changed.prefs.work).toEqual({ locale: "en-US", timezone: "America/New_York" });
    expect(changed.prefs.travel).toBeUndefined();
    const filtered = JSON.parse((await api.handle("GET", `/contacts/?firstName=Ada&lastName=Byron&id=${saved.id}`, undefined)).body) as Contact[];
    expect(filtered).toHaveLength(1);
    expect(filtered[0]?.settings).toEqual({ theme: "light", locale: "en" });
    expect(filtered[0]?.addresses).toHaveLength(2);
    expect((await api.handle("DELETE", `/contacts/?id=${saved.id}`, undefined)).status).toBe(204);
    const remaining = JSON.parse((await api.handle("get", "/contacts/", undefined)).body) as Contact[];
    expect(remaining).toHaveLength(1);
    expect(remaining[0]?.firstName).toBe("Grace");
    expect(remaining[0]?.addresses).toHaveLength(2);
    expect(remaining[0]?.settings).toEqual({ imported: "yes", source: "csv" });
    const legacy = await api.handle("POST", "/legacy_contacts/", JSON.stringify({
      key: "ada",
      firstName: "Ada",
      lastName: "Lovelace",
      email: "ada@example.com",
      importedAt: "2024-06-01T00:00:00Z",
    }));
    expect(legacy.status).toBe(201);
    expect((JSON.parse((await api.handle("GET", "/legacy_contacts/?firstName=Ada", undefined)).body) as { key: string }[])[0]?.key).toBe("ada");
    const legacyPut = await api.handle("PUT", "/legacy_contacts/?key=ada", JSON.stringify({ firstName: "Augusta", lastName: "Lovelace", email: "" }));
    expect(legacyPut.status).toBe(200);
    expect(legacyPut.body).toContain("Augusta");
    expect((await api.handle("DELETE", "/legacy_contacts/?key=ada", undefined)).status).toBe(204);
    expect(JSON.parse((await api.handle("GET", "/legacy_contacts", undefined)).body)).toHaveLength(0);
    if (record) {
      mkdirSync(shared, { recursive: true });
      writeFileSync(logFile, `${lines.join("\n")}\n`);
    } else (middleware as ExpectConsoleMiddleware).complete();
  });

  it("rejects unknown routes lookups and missing rows", async () => {
    const api = await open();
    expect((await api.handle("GET", "/", undefined)).status).toBe(404);
    expect((await api.handle("GET", "", undefined)).status).toBe(404);
    expect((await api.handle("GET", "/contacts?", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/nope", undefined)).status).toBe(404);
    expect((await api.handle("POST", "/contacts/", " ")).status).toBe(400);
    expect((await api.handle("POST", "/contacts/", "null")).status).toBe(400);
    expect((await api.handle("POST", "/contacts/", "{")).status).toBe(400);
    const unknown = await api.handle("POST", "/contacts/", JSON.stringify({ firstName: "Ada", lastName: "Lovelace", contactSourceName: "Nope" }));
    expect(unknown.status).toBe(400);
    expect(unknown.body).toContain("Unknown contact_source");
    expect((await api.handle("POST", "/contacts/", JSON.stringify({ firstName: "Ada", lastName: "Lovelace" }))).status).toBe(400);
    expect((await api.handle("PUT", "/contacts/?id=abc", "{}")).status).toBe(400);
    expect((await api.handle("PUT", "/contacts/?id=99", JSON.stringify({ firstName: "Ada", lastName: "Lovelace", contactSourceName: "Manual" }))).status).toBe(404);
    expect((await api.handle("DELETE", "/contacts/?id=99", undefined)).status).toBe(404);
    expect((await api.handle("DELETE", "/contacts/", undefined)).status).toBe(400);
    expect((await api.handle("POST", "/legacy_contacts/", "{}")).status).toBe(400);
    expect((await api.handle("PUT", "/legacy_contacts/?key=missing", JSON.stringify({ firstName: "A", lastName: "B" }))).status).toBe(404);
    expect((await api.handle("DELETE", "/legacy_contacts/?key=missing", undefined)).status).toBe(404);
    expect((await api.handle("DELETE", "/legacy_contacts/?key=", undefined)).status).toBe(400);
    expect((await api.handle("GET", "/contacts/?id=&lonely&firstName=Ada%20Lovelace", undefined)).status).toBe(200);
    expect((await api.handle("POST", "/contacts/", JSON.stringify({ firstName: "Ada", lastName: "Lovelace", contactSourceName: "Manual" }))).status).toBe(201);
  });

  it("maps datasource errors", async () => {
    const connection = new SqliteDriver().connection({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    const contacts = new ContactRepository(connection);
    const api = new Api(contacts, new LegacyContactRepository(connection));
    expect((await api.handle("GET", "/contacts", undefined)).status).toBe(400);
  });

  it("rethrows unexpected errors", async () => {
    await expect(
      new Api(
        { read: () => { throw new Error("boom"); } } as never,
        {} as never,
      ).handle("GET", "/contacts", undefined),
    ).rejects.toThrow("boom");
  });
});
