import { beforeEach, describe, expect, it } from "vitest";
import {
  ChangeCasingMapper,
  Datasource,
  DatasourceFactory,
  FlatToNestedMapper,
  Native,
  ReturnStatement,
  MetaDataRowTypeConverter,
  StatementTextRenderer,
  TransformMapper,
} from "@datasources/core";
import {
  loadDialect,
  runSpec,
  specFiles,
} from "../../../sql-spec-runner.ts";
import { SqliteCommand } from "../src/command.ts";
import { SqliteConnection } from "../src/connection.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";
import { SqliteStatementRenderer } from "../src/SqliteStatementRenderer.ts";
import { applySchema } from "../../../schema-spec.ts";

const dialect = loadDialect(
  "sqlite",
  (statement) => new StatementTextRenderer(t => new SqliteStatementRenderer(t)).render(statement),
);

class NameValue {
  name = "";
  value = "";
}

class Item {
  id = 0;
  name = "";
}

class Contact {
  name = "";
}

class User {
  id = 0;
  username = "";
  email = "";
  contact_name1 = "";
  contact_name2 = "";
  contacts: Contact[] = [];
}

describe("shared sql specs", () => {
  for (const file of specFiles()) {
    it(file.split("/").pop()!, async () => {
      await runSpec(file, dialect, {
        openSession: async (filename) => {
          const connection = SqliteConnection.from({
            DATASOURCE_URL: filename === ":memory:" ? "sqlite:///:memory:" : `sqlite:///${filename}`,
          });
          return { connection, command: new SqliteCommand(connection) };
        },
        closedCommand: () =>
          new SqliteCommand(new SqliteConnection()),
      });
    });
  }
});

describe("command", () => {
  beforeEach(() => {
    DatasourceFactory.reset();
    DatasourceFactory.register("sqlite", new SqliteDriver());
    delete process.env.INSTALLED_PROVIDERS;
    delete process.env.DATASOURCE_URL;
    process.env.PROVIDER = "sqlite";
  });

  it("reads a boolean column as bool and leaves an integer", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await applySchema(connection, "flags");
    await command.executeNative(new Native("INSERT INTO flags (enabled, count) VALUES (1, 1)"));
    await using reader = await command.executeReaderNative(new Native("SELECT enabled, count, 1 AS one FROM flags"));
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ enabled: true, count: 1, one: 1 });
  });

  it("run executeReader on a simple list", async () => {
    const ds = Datasource.default;
    const command = await ds.createCommand();
    await applySchema(command.connection, "items");
    await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (1, 'ada')"));
    await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (2, 'grace')"));
    await command.executeNative(new Native("INSERT INTO items (id, name) VALUES (3, 'bea')"));
    await using reader = await command.executeReader("SELECT id, name FROM items");
    const rows = await Array.fromAsync(reader.read());
    expect(rows).toEqual([
      { id: 1, name: "ada" },
      { id: 2, name: "grace" },
      { id: 3, name: "bea" },
    ]);
  });

  it("read and apply case mappings", async () => {
    const ds = Datasource.default;
    const command = await ds.createCommand();
    await applySchema(command.connection, "case-contacts");
    await command.executeNative(new Native("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')"));
    await using reader = await command.executeReader("SELECT user_id, first_name FROM contacts");
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const mapped = await Array.fromAsync(reader.read(), (source) =>
      Object.fromEntries(Object.entries(source).map(([key, value]) => [casing.reverseMap(key), value])),
    );
    expect(mapped).toEqual([{ UserId: 1, FirstName: "Ada" }]);
  });

  it("executeReader, then apply flat to nested mapper", async () => {
    const ds = Datasource.default;
    const command = await ds.createCommand();
    await applySchema(command.connection, "flat-users");
    await command.executeNative(new Native("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')"));
    await command.executeNative(new Native("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')"));
    await using reader = await command.executeReader("SELECT id, username, contact_name FROM users");
    const rows = await Array.fromAsync(reader.read());
    const mapper = new FlatToNestedMapper<{
      id: number;
      username: string;
      contacts: { name: string }[];
    }>({
      key: "id",
      fields: {
        id: "id",
        username: "username",
        contacts: { key: "contact_name", fields: { name: "contact_name" } },
      },
    });
    const nested = mapper.map(rows);
    expect(nested).toEqual([
      { id: 1, username: "bobby", contacts: [{ name: "bob" }, { name: "roberta" }] },
    ]);
  });

  it("run executeReader and return NameValues", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await applySchema(connection, "settings");
    await command.executeNative(new Native("INSERT INTO settings (name, value) VALUES ('theme', 'dark')"));
    await command.executeNative(new Native("INSERT INTO settings (name, value) VALUES ('lang', 'en')"));
    await command.executeNative(new Native("INSERT INTO settings (name, value) VALUES ('size', 'lg')"));
    await using reader = await command.executeReader("SELECT name, value FROM settings");
    const converter = new MetaDataRowTypeConverter<NameValue>();
    const items = await Array.fromAsync(reader.read(), (row) => converter.convert(row, NameValue));
    expect(items).toEqual([
      { name: "theme", value: "dark" },
      { name: "lang", value: "en" },
      { name: "size", value: "lg" },
    ]);
  });

  it("execute, insert, read, update, and delete", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await applySchema(connection, "items");
    await command.execute("INSERT INTO items (id, name) VALUES ($id, $name)", [
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ]);
    const converter = new MetaDataRowTypeConverter<Item>();
    await using createdReader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
      { name: "id", value: 1 },
    ]);
    const created = await Array.fromAsync(createdReader.read(), (row) => converter.convert(row, Item));
    const response = await command.execute("UPDATE items SET name = $name WHERE id = $id", [
      { name: "id", value: 1 },
      { name: "name", value: "A" },
    ]);
    await using updatedReader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
      { name: "id", value: 1 },
    ]);
    const updated = await Array.fromAsync(updatedReader.read(), (row) => converter.convert(row, Item));
    await command.execute("DELETE FROM items WHERE id = $id", [{ name: "id", value: 1 }]);
    await using goneReader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
      { name: "id", value: 1 },
    ]);
    const gone = await Array.fromAsync(goneReader.read(), (row) => converter.convert(row, Item));
    expect(created).toEqual([{ id: 1, name: "a" }]);
    expect(response.rowsAffected).toBe(1);
    expect(updated).toEqual([{ id: 1, name: "A" }]);
    expect(gone).toEqual([]);
  });

  it("maps flat contacts", async () => {
    const ds = Datasource.default;
    const command = await ds.createCommand();
    await applySchema(command.connection, "flat-contact-users");
    await command.executeNative(
      new Native(
        "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')",
      ),
    );
    await using reader = await command.executeReader(
      "SELECT contact_name1, contact_name2, email, id, username FROM users",
    );
    const converter = new MetaDataRowTypeConverter<User>();
    const userContactsMapper = () =>
      new TransformMapper<User, User>(
        (row) => {
          row.contacts = [
            Object.assign(new Contact(), { name: row.contact_name1 }),
            Object.assign(new Contact(), { name: row.contact_name2 }),
          ];
          return row;
        },
        (user) => {
          user.contact_name1 = user.contacts[0]?.name ?? "";
          user.contact_name2 = user.contacts[1]?.name ?? "";
          return user;
        },
      );
    const mapper = userContactsMapper();
    const users = await Array.fromAsync(reader.read(), (row) => mapper.map(converter.convert(row, User)));
    expect(users[0]?.username).toBe("bobby");
    expect(users[0]?.email).toBe("bob@asdf.com");
    expect(users[0]?.contacts.map((contact) => contact.name)).toEqual(["bob", "roberta"]);
  });

  it("reads multiple results returned in a single command", async () => {
    const ds = Datasource.default;
    const command = await ds.createCommand();
    await using reader = await command.executeReaderNative(new Native("SELECT 1 AS id; SELECT 2 AS id"));
    const first: { id: number }[] = [];
    while (await reader.next()) first.push(reader.current as { id: number });
    const moved = await reader.nextResult();
    const second: { id: number }[] = [];
    while (await reader.next()) second.push(reader.current as { id: number });
    const again = await reader.nextResult();
    expect(moved).toBe(true);
    expect(again).toBe(false);
    expect(first[0]?.id).toBe(1);
    expect(second[0]?.id).toBe(2);
  });
});

describe("return statement", () => {
  it("selects a scalar", async () => {
    const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    const command = new SqliteCommand(connection);
    const parameters = [{ name: "x", value: -3 }];
    const response = await command.executeStatement(
      new ReturnStatement("abs", parameters),
      parameters,
    );
    expect(response.result).toBe(3);
  });
});
