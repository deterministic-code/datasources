import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@deterministic-code/datasources-memory";
import { collectAsync } from "./collectAsync.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import type { IConnection } from "./IConnection.ts";
import { ChangeCasingMapper } from "./mappers/ChangeCasingMapper.ts";
import { Schema } from "./Schema.ts";

class NamedType {
  Id = 0;
  Name = "";
}

class Address {
  Id = 0;
  ContactId = 0;
  Line1 = "";
  AddressTypeId = 0;
  AddressTypeName = "";
}

class Setting {
  Id = 0;
  ContactId = 0;
  Key = "";
  Value = "";
}

class Contact {
  Id = 0;
  ContactSourceId = 0;
  ContactSourceName = "";
  Addresses: Address[] = [];
  Settings: Record<string, string> = {};
}

class BigId {
  Id = 0;
  Name = "";
}

const columns = new ChangeCasingMapper("pascalCase", "snakeCase");

const types = (connection: IConnection, entity: string) =>
  new CRUDRepository<NamedType>(connection, {
    type: NamedType,
    entity,
    ids: ["Id"],
    propertyMappings: columns,
  });

const addresses = (connection: IConnection) =>
  new CRUDRepository<Address>(connection, {
    type: Address,
    ids: ["Id"],
    propertyMappings: columns,
    readProperties: ["Id", "ContactId", "Line1", "AddressTypeId"],
    createProperties: ["ContactId", "Line1"],
    schema: new Schema({
      Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
      ContactId: { type: "integer", nullable: false, hasDefault: false },
      Line1: { type: "string", nullable: false, hasDefault: false },
      AddressTypeName: { type: "string", nullable: false, hasDefault: false },
    }),
  });

const graph = async () => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  const sources = types(connection, "ContactSource");
  const addressTypes = types(connection, "AddressType");
  await sources.create(Object.assign(new NamedType(), { Name: "import" }));
  await addressTypes.create(Object.assign(new NamedType(), { Name: "Home" }));
  const addressRepo = addresses(connection);
  addressRepo.addEnriched(
    { destinationProperty: "AddressTypeName", sourceProperty: "AddressTypeId", relatedKey: "Id", relatedProperty: "Name" },
    addressTypes,
  );
  const settings = new CRUDRepository<Setting>(connection, {
    type: Setting,
    ids: ["Id"],
    propertyMappings: columns,
    schema: new Schema({
      Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
      ContactId: { type: "integer", nullable: false, hasDefault: false },
      Key: { type: "string", nullable: false, hasDefault: false },
      Value: { type: "string", nullable: false, hasDefault: false },
    }),
  });
  const contacts = new CRUDRepository<Contact>(connection, {
    type: Contact,
    ids: ["Id"],
    propertyMappings: columns,
    readProperties: ["Id", "ContactSourceId"],
    createProperties: [],
    schema: new Schema({
      Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
      ContactSourceName: { type: "string", nullable: false, hasDefault: false },
    }),
  });
  contacts.addEnriched(
    { destinationProperty: "Addresses", sourceProperty: "Id", relatedKey: "ContactId", type: "list" },
    addressRepo,
  );
  contacts.addEnriched(
    { destinationProperty: "Settings", sourceProperty: "Id", relatedKey: "ContactId", relatedProperty: "Value", type: "dictionary" },
    settings,
  );
  contacts.addEnriched(
    { destinationProperty: "ContactSourceName", sourceProperty: "ContactSourceId", relatedKey: "Id", relatedProperty: "Name" },
    sources,
  );
  return { contacts, addresses: addressRepo };
};

describe("schema enrichment", () => {
  it("create-resolves-address-type-name-without-address-type-id-in-schema", async () => {
    const connection = new MemoryDriver().connection();
    await connection.open();
    const addressTypes = types(connection, "AddressType");
    const addressRepo = addresses(connection);
    const home = await addressTypes.create(Object.assign(new NamedType(), { Name: "Home" }));
    addressRepo.addEnriched(
      { destinationProperty: "AddressTypeName", sourceProperty: "AddressTypeId", relatedKey: "Id", relatedProperty: "Name" },
      addressTypes,
    );
    const created = await addressRepo.create(Object.assign(new Address(), { Line1: "1 Main", AddressTypeName: "Home" }));
    expect(created.Id).toBeGreaterThan(0);
    const loaded = await addressRepo.readSingle();
    expect(loaded?.Line1).toBe("1 Main");
    expect(loaded?.AddressTypeName).toBe("Home");
    expect(loaded?.AddressTypeId).toBe(home.Id);
  });

  it("create-keeps-address-type-id-when-name-is-empty", async () => {
    const connection = new MemoryDriver().connection();
    await connection.open();
    const addressTypes = types(connection, "AddressType");
    const addressRepo = addresses(connection);
    const home = await addressTypes.create(Object.assign(new NamedType(), { Name: "Home" }));
    addressRepo.addEnriched(
      { destinationProperty: "AddressTypeName", sourceProperty: "AddressTypeId", relatedKey: "Id", relatedProperty: "Name" },
      addressTypes,
    );
    const created = await addressRepo.create(Object.assign(new Address(), { Line1: "1 Main", AddressTypeId: home.Id }));
    expect(created.Id).toBeGreaterThan(0);
    expect((await addressRepo.readSingle())?.AddressTypeId).toBe(home.Id);
  });

  it("create-contact-stamps-child-contact-id-and-resolves-names", async () => {
    const { contacts, addresses: addressRepo } = await graph();
    const created = await contacts.create(Object.assign(new Contact(), {
      ContactSourceName: "import",
      Addresses: [Object.assign(new Address(), { Line1: "1 Main", AddressTypeName: "Home" })],
      Settings: { theme: "dark" },
    }));
    expect(created.Id).toBeGreaterThan(0);
    const loaded = await contacts.readSingle();
    expect(loaded?.ContactSourceName).toBe("import");
    expect(loaded?.Settings.theme).toBe("dark");
    expect(loaded?.Addresses).toHaveLength(1);
    expect(loaded?.Addresses[0]?.Line1).toBe("1 Main");
    expect(loaded?.Addresses[0]?.AddressTypeName).toBe("Home");
    expect(loaded?.Addresses[0]?.ContactId).toBe(created.Id);
    expect((await collectAsync(addressRepo.read()))[0]?.ContactId).toBe(created.Id);
  });

  it("create-unknown-contact-source-name-throws", async () => {
    const { contacts } = await graph();
    await expect(contacts.create(Object.assign(new Contact(), { ContactSourceName: "Nope" }))).rejects.toThrow(
      /ContactSourceName/,
    );
  });

  it("create-missing-contact-source-name-throws", async () => {
    const { contacts } = await graph();
    await expect(contacts.create(new Contact())).rejects.toThrow(/ContactSourceName/);
  });

  it("create-unknown-nested-address-type-name-throws", async () => {
    const { contacts } = await graph();
    await expect(
      contacts.create(Object.assign(new Contact(), {
        ContactSourceName: "import",
        Addresses: [Object.assign(new Address(), { Line1: "1 Main", AddressTypeName: "Nope" })],
      })),
    ).rejects.toThrow(/AddressTypeName/);
  });

  it("create-keeps-unknown-contact-source-id-when-name-is-empty", async () => {
    const { contacts } = await graph();
    const created = await contacts.create(Object.assign(new Contact(), { ContactSourceId: 99 }));
    expect(created.Id).toBeGreaterThan(0);
    expect((await contacts.readSingle())?.ContactSourceId).toBe(99);
  });

  it("create-writes-biginteger-id", async () => {
    const connection = new MemoryDriver().connection();
    await connection.open();
    const repo = new CRUDRepository<BigId>(connection, {
      type: BigId,
      ids: ["Id"],
      schema: new Schema({
        Id: { type: "biginteger", nullable: false, hasDefault: false, isId: true },
        Name: { type: "string", nullable: false, hasDefault: false },
      }),
    });
    const created = await repo.create(Object.assign(new BigId(), { Name: "Ada" }));
    expect(created.Id).toBeGreaterThan(0);
    expect((await repo.readSingle())?.Name).toBe("Ada");
  });
});
