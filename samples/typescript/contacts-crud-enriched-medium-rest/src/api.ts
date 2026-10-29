import { DatasourceError, Operator, Where, type IWhere } from "@deterministic-code/datasources";
import type { Contact } from "../generated/typescript/types/generated/views/contact.ts";
import type { Address } from "../generated/typescript/types/generated/views/address.ts";
import type { Phone } from "../generated/typescript/types/generated/views/phone.ts";
import type { LegacyContact } from "../generated/typescript/types/generated/views/legacyContact.ts";
import { ContactRepository } from "../generated/typescript/repositories/contact-repository.ts";
import { clock } from "./clock.ts";
import { LegacyContactRepository } from "../generated/typescript/repositories/legacy-contact-repository.ts";

export type ContactWrite = Contact;
export type HttpResult = { status: number; body: string };

export class Api {
  constructor(
    private readonly contacts: ContactRepository,
    private readonly legacy: LegacyContactRepository,
  ) {}

  handle = async (method: string, target: string, body: string | undefined): Promise<HttpResult> => {
    try {
      const split = target.indexOf("?");
      const path = (split < 0 ? target : target.slice(0, split)).replace(/\/$/, "") || "/";
      const query = parse(split < 0 ? "" : target.slice(split + 1));
      return await ({
        "GET /contacts": () => this.getContacts(query),
        "POST /contacts": () => this.postContact(body),
        "PUT /contacts": () => this.putContact(query, body),
        "DELETE /contacts": () => this.deleteContact(query),
        "GET /legacy_contacts": () => this.getLegacy(query),
        "POST /legacy_contacts": () => this.postLegacy(body),
        "PUT /legacy_contacts": () => this.putLegacy(query, body),
        "DELETE /legacy_contacts": () => this.deleteLegacy(query),
      }[`${method.toUpperCase()} ${path}`] ?? (() => Promise.resolve({ status: 404, body: "" })))();
    } catch (error) {
      return error instanceof RequestError
        ? { status: error.status, body: error.message }
        : error instanceof SyntaxError
            || error instanceof DatasourceError
            || error instanceof TypeError
            || (error instanceof Error && error.name === "SqliteError")
          ? { status: 400, body: error.message }
          : Promise.reject(error);
    }
  };

  private getContacts = async (query: Record<string, string>): Promise<HttpResult> =>
    ({ status: 200, body: JSON.stringify(await list(this.contacts, where(query, ["id", "firstName", "lastName", "email"]))) });

  private postContact = async (body: string | undefined): Promise<HttpResult> => {
    const contact = this.stamp(read<ContactWrite>(body));
    return { status: 201, body: JSON.stringify(await this.contacts.create(contact)) };
  };

  private putContact = async (query: Record<string, string>, body: string | undefined): Promise<HttpResult> => {
    const id = requiredNumber(query, "id");
    if ((await this.contacts.readSingle(new Where(new Operator.Equals("id", id)))) == null) return { status: 404, body: "" };
    const contact = this.stamp(read<ContactWrite>(body));
    contact.id = id;
    await this.contacts.update(contact);
    return { status: 200, body: JSON.stringify(await this.contacts.readSingle(new Where(new Operator.Equals("id", id)))) };
  };

  private deleteContact = async (query: Record<string, string>): Promise<HttpResult> => {
    const id = requiredNumber(query, "id");
    const contact = await this.contacts.readSingle(new Where(new Operator.Equals("id", id)));
    if (contact == null) return { status: 404, body: "" };
    await this.contacts.delete(contact);
    return { status: 204, body: "" };
  };

  private getLegacy = async (query: Record<string, string>): Promise<HttpResult> =>
    ({ status: 200, body: JSON.stringify(await list(this.legacy, where(query, ["key", "firstName", "lastName", "email"]))) });

  private postLegacy = async (body: string | undefined): Promise<HttpResult> => {
    const row = textLegacy(read<LegacyContact>(body));
    if (row.key.length === 0) throw new RequestError("key is required");
    return { status: 201, body: JSON.stringify(await this.legacy.create(row)) };
  };

  private putLegacy = async (query: Record<string, string>, body: string | undefined): Promise<HttpResult> => {
    const key = required(query, "key");
    if ((await this.legacy.readSingle(new Where(new Operator.Equals("key", key)))) == null) return { status: 404, body: "" };
    const row = textLegacy(read<LegacyContact>(body));
    row.key = key;
    await this.legacy.update(row, new Where(new Operator.Equals("key", key)));
    return { status: 200, body: JSON.stringify(await this.legacy.readSingle(new Where(new Operator.Equals("key", key)))) };
  };

  private deleteLegacy = async (query: Record<string, string>): Promise<HttpResult> => {
    const key = required(query, "key");
    const row = await this.legacy.readSingle(new Where(new Operator.Equals("key", key)));
    if (row == null) return { status: 404, body: "" };
    await this.legacy.delete(row, new Where(new Operator.Equals("key", key)));
    return { status: 204, body: "" };
  };

  private stamp = (contact: ContactWrite): ContactWrite => {
    contact.email = text(contact.email);
    contact.notes = text(contact.notes) || "none";
    stampContact(contact);
    contact.addresses ??= [];
    contact.phones ??= [];
    contact.settings ??= {};
    contact.prefs ??= {};
    for (const address of contact.addresses) stampAddress(address);
    for (const phone of contact.phones) stampPhone(phone);
    return contact;
  };
}

class RequestError extends Error {
  constructor(
    message: string,
    readonly status = 400,
  ) {
    super(message);
  }
}

const stampContact = (contact: ContactWrite): void => {
  contact.uuid = contact.uuid || clock.uuid();
  contact.created = contact.created || clock.now();
  contact.updated = clock.now();
  contact.version = contact.version || "AA==";
};

const stampAddress = (address: Address): void => {
  address.line1 = text(address.line1);
  address.line2 = text(address.line2);
  address.city = text(address.city);
  address.region = text(address.region);
  address.postalCode = text(address.postalCode);
  address.country = text(address.country) || "US";
  address.uuid = address.uuid || clock.uuid();
  address.created = address.created || clock.now();
  address.updated = clock.now();
  address.version = address.version || "AA==";
};

const stampPhone = (phone: Phone): void => {
  phone.number = text(phone.number);
  phone.label = text(phone.label) || "Main";
  phone.uuid = phone.uuid || clock.uuid();
  phone.created = phone.created || clock.now();
  phone.updated = clock.now();
  phone.version = phone.version || "AA==";
};

const list = async <T>(repo: { read: (query?: IWhere) => AsyncIterable<T> }, query: IWhere | undefined): Promise<T[]> => {
  const rows: T[] = [];
  for await (const row of query == null ? repo.read() : repo.read(query)) rows.push(row);
  return rows;
};

const where = (query: Record<string, string>, keys: string[]): IWhere | undefined => {
  const parts = keys
    .filter((key) => (query[key] ?? "").length > 0)
    .map((key) => new Operator.Equals(key, key === "id" ? Number(query[key]) : query[key]));
  return parts.length === 0 ? undefined : new Where(parts.reduce((left, right) => new Operator.And(left, right)));
};

const read = <T>(body: string | undefined): T => {
  if (body == null || body.trim() === "") return {} as T;
  const parsed: unknown = JSON.parse(body);
  if (parsed == null || typeof parsed !== "object") throw new SyntaxError("invalid json");
  return parsed as T;
};

const textLegacy = (row: LegacyContact): LegacyContact => ({
  ...row,
  key: text(row.key),
  firstName: text(row.firstName),
  lastName: text(row.lastName),
  email: text(row.email),
  importedAt: row.importedAt ?? clock.now(),
});

const text = (value: string | null | undefined): string => value ?? "";

const requiredNumber = (query: Record<string, string>, key: string): number => {
  const value = Number(required(query, key));
  if (Number.isNaN(value)) throw new TypeError(`${key} is required`);
  return value;
};

const required = (query: Record<string, string>, key: string): string =>
  (query[key] ?? "").length > 0 ? query[key]! : (() => { throw new RequestError(`${key} is required`); })();

const parse = (query: string): Record<string, string> =>
  Object.fromEntries(
    query
      .split("&")
      .filter((part) => part.length > 0)
      .map((part) => {
        const eq = part.indexOf("=");
        return [
          decodeURIComponent(eq < 0 ? part : part.slice(0, eq)),
          decodeURIComponent(eq < 0 ? "" : part.slice(eq + 1)),
        ];
      }),
  );
