import { DatasourceError } from "@deterministic-code/datasources";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { describe, expect, it } from "vitest";
import { Api, type ContactWrite } from "./api.ts";
import "./datasources-register.ts";
import { LegacyContactRepository } from "../generated/typescript/repositories/legacy-contact-repository.ts";
import { ContactRepository } from "../generated/typescript/repositories/contact-repository.ts";
import { ContactGroupRepository } from "../generated/typescript/repositories/contact-group-repository.ts";

const lookup = (id?: number) => ({
  readSingle: async () => (id == null ? undefined : { id }),
});

const contacts = (opts?: {
  read?: object[];
  single?: object | undefined;
  create?: object;
  source?: number;
  address?: number;
  phone?: number;
}) => {
  const id = (key: "source" | "address" | "phone") =>
    opts != null && Object.hasOwn(opts, key) ? opts[key] : 1;
  return {
    read: async function* () {
      for (const row of opts?.read ?? []) yield row;
    },
    readSingle: async () => opts?.single,
    create: async (row: ContactWrite) => {
      if (id("source") == null)
        throw new DatasourceError(`Unknown contact_source "${row.contactSourceName ?? ""}"`);
      return opts?.create ?? { ...row, id: 1 };
    },
    update: async () => undefined,
    delete: async () => undefined,
    contactSource: lookup(id("source")),
    address: { addressType: lookup(id("address")) },
    phone: { phoneType: lookup(id("phone")) },
  } as never;
};

const legacy = (opts?: { read?: object[]; single?: object | undefined; create?: object }) =>
  ({
    read: async function* () {
      for (const row of opts?.read ?? []) yield row;
    },
    readSingle: async () => opts?.single,
    create: async (row: object) => opts?.create ?? row,
    update: async () => undefined,
    delete: async () => undefined,
  }) as never;

const groups = (opts?: { read?: object[]; single?: object | undefined; create?: object }) =>
  ({
    read: async function* () {
      for (const row of opts?.read ?? []) yield row;
    },
    readSingle: async () => opts?.single,
    create: async (row: object) => opts?.create ?? { ...row, id: 1 },
    update: async () => undefined,
    delete: async () => undefined,
  }) as never;

describe("api", () => {
  it("maps datasource errors", async () => {
    const connection = new SqliteDriver().connection({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    const api = new Api(new ContactRepository(connection), new LegacyContactRepository(connection), new ContactGroupRepository(connection));
    expect((await api.handle("GET", "/contacts", undefined)).status).toBe(400);
  });

  it("rethrows unexpected errors", async () => {
    await expect(
      new Api({ read: () => { throw new Error("boom"); } } as never, {} as never, groups()).handle("GET", "/contacts", undefined),
    ).rejects.toThrow("boom");
  });

  it("rejects unknown routes lookups and missing rows", async () => {
    const api = new Api(contacts({ source: undefined }), legacy(), groups());
    expect((await api.handle("GET", "/", undefined)).status).toBe(404);
    expect((await api.handle("GET", "", undefined)).status).toBe(404);
    expect((await api.handle("GET", "/contacts?", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/nope", undefined)).status).toBe(404);
    expect((await api.handle("POST", "/contacts/", " ")).status).toBe(400);
    expect((await api.handle("POST", "/contacts/", "null")).status).toBe(400);
    expect((await api.handle("POST", "/contacts/", "{")).status).toBe(400);
    const unknown = await api.handle("POST", "/contacts/", JSON.stringify({ firstName: "Ada", lastName: "Lovelace", contactSourceName: "Nope" }));
    expect(unknown.status).toBe(400);
    expect(unknown.body).toBe('Unknown contact_source "Nope"');
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
    expect((await api.handle("GET", "/contacts/?id=1&firstName=Ada", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/contact_groups", undefined)).status).toBe(200);
    expect((await api.handle("PUT", "/contact_groups/?id=99", JSON.stringify({ name: "Coverage" }))).status).toBe(404);
    expect((await api.handle("PUT", "/contact_groups/", JSON.stringify({ name: "Coverage" }))).status).toBe(400);
  });

  it("writes and reads contacts and legacy contacts", async () => {
    const created = { id: 1, firstName: "Ada", addresses: [], phones: [], settings: {}, prefs: {} };
    const api = new Api(
      contacts({
        create: created,
        single: created,
        read: [created],
        source: 1,
        address: 1,
        phone: 1,
      }),
      legacy({ create: { key: "ada", firstName: "Ada" }, single: { key: "ada", firstName: "Augusta" }, read: [] }),
      groups({ create: { id: 1, name: "Coverage", members: [] }, single: { id: 1, name: "Coverage", members: [] }, read: [{ id: 1, name: "Coverage", members: [] }] }),
    );
    expect((await api.handle("POST", "/contacts/", JSON.stringify({
      firstName: "Ada",
      lastName: "Lovelace",
      contactSourceName: "Manual",
      addresses: [{ line1: "1", city: "London", addressTypeName: "Home" }],
      phones: [{ number: "1", phoneTypeName: "Mobile" }],
    }))).status).toBe(201);
    expect((await api.handle("PUT", "/contacts/?id=1", JSON.stringify({
      firstName: "Ada",
      lastName: "Byron",
      contactSourceName: "Manual",
    }))).status).toBe(200);
    expect((await api.handle("DELETE", "/contacts/?id=1", undefined)).status).toBe(204);
    expect((await api.handle("POST", "/legacy_contacts/", JSON.stringify({ key: "ada", firstName: "Ada", lastName: "Lovelace" }))).status).toBe(201);
    expect((await api.handle("PUT", "/legacy_contacts/?key=ada", JSON.stringify({ firstName: "Augusta", lastName: "Lovelace" }))).status).toBe(200);
    expect((await api.handle("DELETE", "/legacy_contacts/?key=ada", undefined)).status).toBe(204);
    expect((await api.handle("GET", "/legacy_contacts", undefined)).status).toBe(200);
    expect((await api.handle("GET", "/contacts", undefined)).status).toBe(200);
    expect((await api.handle("POST", "/contact_groups/", JSON.stringify({ name: "Coverage", members: [{ id: 1 }] }))).status).toBe(201);
    expect((await api.handle("PUT", "/contact_groups/?id=1", JSON.stringify({ name: "Coverage", members: [{ id: 1 }] }))).status).toBe(200);
    expect((await api.handle("GET", "/contact_groups", undefined)).status).toBe(200);
    expect(new DatasourceError("x").message).toBe("x");
  });
});
