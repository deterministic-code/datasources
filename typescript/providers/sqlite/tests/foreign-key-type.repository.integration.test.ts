import { describe, expect, it } from "vitest";
import {
  CRUDRepository,
  ChangeCasingMapper,
  Datasource,
  Native,
  Schema,
  createCRUDRepositoryOptions,
  type IConnection,
  type SchemaColumn,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "../src/SqliteDriver.ts";

class LongContact {
  Id = 0;
  Name = "";
}

class LongAddress {
  Id = 0;
  ContactId = 0;
  City = "";
}

class GuidContact {
  Id = "";
  Name = "";
}

class GuidAddress {
  Id = 0;
  ContactId = "";
  City = "";
}

const ada = "11111111-1111-1111-1111-111111111111";

const open = async (): Promise<IConnection> => {
  const ds = new Datasource("sqlite", new SqliteDriver(), { DATASOURCE_URL: "sqlite:///:memory:" });
  const connection = await ds.createConnection();
  await connection.open();
  return connection;
};

const column = (type: string, isId = false): SchemaColumn => ({
  type,
  nullable: false,
  hasDefault: false,
  isId,
});

const repos = async <TParent extends object, TChild extends object>(
  connection: IConnection,
  Parent: new () => TParent,
  Child: new () => TChild,
  id: SchemaColumn,
  createProperties?: string[],
) => {
  const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
  return {
    contactsRepo: new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(Parent),
      entity: "contacts",
      propertyMappings: casing,
      ids: ["Id"],
      schema: new Schema({
        Id: { ...id, isId: true },
        Name: column("string"),
      }),
      createProperties,
    }),
    addressesRepo: new CRUDRepository(connection, {
      ...createCRUDRepositoryOptions(Child),
      entity: "addresses",
      propertyMappings: casing,
      ids: ["Id"],
      schema: new Schema({
        Id: column("integer", true),
        ContactId: { ...id, isId: false },
        City: column("string"),
      }),
    }),
  };
};

describe("foreign key type", () => {
  it("keeps a long parent id on the child foreign key", async () => {
    const connection = await open();
    const command = connection.createCommand();
    await command.executeNative(
      new Native("CREATE TABLE contacts (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)"),
    );
    await command.executeNative(
      new Native(
        "CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTOINCREMENT, contact_id INTEGER NOT NULL, city TEXT NOT NULL)",
      ),
    );
    const { contactsRepo, addressesRepo } = await repos(connection, LongContact, LongAddress, column("integer"));
    const parent = await contactsRepo.create(Object.assign(new LongContact(), { Name: "Ada" }));
    const child = await addressesRepo.create(
      Object.assign(new LongAddress(), { ContactId: parent.Id, City: "London" }),
    );
    const loaded = await addressesRepo.readSingleById(child.Id);
    const fk: LongContact["Id"] = child.ContactId;
    expect(fk).toBe(parent.Id);
    expect(loaded?.ContactId).toBe(parent.Id);
  });

  it("keeps a guid parent id on the child foreign key", async () => {
    const connection = await open();
    const command = connection.createCommand();
    await command.executeNative(new Native("CREATE TABLE contacts (id TEXT PRIMARY KEY, name TEXT NOT NULL)"));
    await command.executeNative(
      new Native(
        "CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTOINCREMENT, contact_id TEXT NOT NULL, city TEXT NOT NULL)",
      ),
    );
    const { contactsRepo, addressesRepo } = await repos(
      connection,
      GuidContact,
      GuidAddress,
      column("uuid"),
      ["Id", "Name"],
    );
    const parent = await contactsRepo.create(Object.assign(new GuidContact(), { Id: ada, Name: "Ada" }));
    const child = await addressesRepo.create(
      Object.assign(new GuidAddress(), { ContactId: parent.Id, City: "London" }),
    );
    const loaded = await addressesRepo.readSingleById(child.Id);
    const fk: GuidContact["Id"] = child.ContactId;
    expect(fk).toBe(parent.Id);
    expect(loaded?.ContactId).toBe(parent.Id);
    expect(parent.Id).toBe(ada);
  });
});
