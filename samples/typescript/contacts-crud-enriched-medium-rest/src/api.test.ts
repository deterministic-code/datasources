import { ConsoleCommandMiddleware, DatasourceError, ExpectConsoleMiddleware, type ICommandMiddleware } from "@datasources/core";
import { SqliteDriver } from "@datasources/sqlite";
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
      return duration >= 0 && (line.endsWith("ms") || line.endsWith("μs")) ? line.slice(0, duration + 1) : line;
    };
    const middleware = record
      ? new ConsoleCommandMiddleware((line) => lines.push(signature(line)), () => stamp, { isVerbose: "true" })
      : new ExpectConsoleMiddleware(
          readFileSync(logFile, "utf8").split("\n").filter((line) => line.length > 0),
          () => stamp,
        );
    clock.now = () => stamp;
    clock.uuid = () => `00000000-0000-0000-0000-${String(++n).padStart(12, "0")}`;
    const api = await open(middleware);
    const created = await api.handle("POST", "/contacts/", JSON.stringify({
      firstName: "Ada",
      lastName: "Lovelace",
      email: "ada@example.com",
      contactSourceName: "Manual",
      addresses: [{ line1: "12 Analytical Engine", city: "London" }],
      phones: [{ number: "555-0100", label: "Mobile" }],
    }));
    expect(created.status).toBe(201);
    expect(created.body).not.toContain("OldContactsTbl");
    const saved = JSON.parse(created.body) as { id: number; firstName: string; contactSourceName: string; addresses: { city: string }[]; phones: { number: string }[] };
    expect(saved.firstName).toBe("Ada");
    expect(saved.contactSourceName).toBe("Manual");
    expect(saved.addresses[0]?.city).toBe("London");
    expect(saved.phones[0]?.number).toBe("555-0100");
    await api.handle("POST", "/contacts", JSON.stringify({
      firstName: "Grace",
      lastName: "Hopper",
      uuid: "22222222-2222-2222-2222-222222222222",
      created: "2024-01-02T00:00:00Z",
      contactSourceName: "Imported",
      addresses: [{ uuid: "33333333-3333-3333-3333-333333333333", created: "2024-01-02T00:00:00Z", line1: "1", city: "NY" }],
      phones: [{ uuid: "44444444-4444-4444-4444-444444444444", created: "2024-01-02T00:00:00Z", number: "1" }],
    }));
    const filtered = await api.handle("GET", `/contacts/?firstName=Ada&lastName=Lovelace&id=${saved.id}`, undefined);
    expect(JSON.parse(filtered.body)).toHaveLength(1);
    const updated = await api.handle("PUT", `/contacts/?id=${saved.id}`, JSON.stringify({
      firstName: "Ada",
      lastName: "Byron",
      email: null,
      contactSourceName: "Manual",
      addresses: [{ line1: "1 Strand", city: "London" }],
      phones: [],
    }));
    expect(updated.status).toBe(200);
    const changed = JSON.parse(updated.body) as { lastName: string; addresses: { line1: string }[]; phones: unknown[] };
    expect(changed.lastName).toBe("Byron");
    expect(changed.addresses[0]?.line1).toBe("1 Strand");
    expect(changed.phones).toHaveLength(0);
    expect((await api.handle("DELETE", `/contacts/?id=${saved.id}`, undefined)).status).toBe(204);
    expect(JSON.parse((await api.handle("get", "/contacts/", undefined)).body)).toHaveLength(1);
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
