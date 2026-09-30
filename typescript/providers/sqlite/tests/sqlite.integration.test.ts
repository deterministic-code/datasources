import { beforeEach, describe, expect, it } from "vitest";
import {
  Datasource,
  DatasourceFactory,
  Native,
  ReturnStatement,
  RowTypeConverter,
  TransformMapper,
} from "@datasources/core";
import {
  loadDialect,
  runSpec,
  specFiles,
} from "../../sql-spec-runner.ts";
import { SqliteCommand } from "../src/command.ts";
import { SqliteConnection } from "../src/connection.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";
import { SqliteStatementFactory } from "../src/SqliteStatementFactory.ts";

const dialect = loadDialect("sqlite", (statement) => new SqliteStatementFactory().transform(statement).getText());

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

function userContactsMapper() {
  return new TransformMapper<User, User>(
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

  it("runs raw sql", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(new Native("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
    await command.execute("INSERT INTO items (id, name) VALUES (1, 'ada')");
    await using reader = await command.executeReader("SELECT id, name FROM items");
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1, name: "ada" });
    expect(await reader.next()).toBe(false);
  });

  it("runs executeReader and returns NameValue", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(new Native("CREATE TABLE settings (name TEXT, value TEXT)"));
    await command.execute("INSERT INTO settings (name, value) VALUES ($name, $value)", [
      { name: "name", value: "theme" },
      { name: "value", value: "dark" },
    ]);
    await using reader = await command.executeReader("SELECT name, value FROM settings");
    const converter = new RowTypeConverter<NameValue>();
    const items: NameValue[] = [];
    while (await reader.next()) items.push(converter.convert(reader.current!, NameValue));
    expect(items[0]?.name).toBe("theme");
    expect(items[0]?.value).toBe("dark");
  });

  it("executes insert update and delete", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(new Native("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)"));
    await command.execute("INSERT INTO items (id, name) VALUES ($id, $name)", [
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ]);
    await command.execute("UPDATE items SET name = $name WHERE id = $id", [
      { name: "id", value: 1 },
      { name: "name", value: "A" },
    ]);
    const converter = new RowTypeConverter<Item>();
    let updated: Item | undefined;
    {
      await using reader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
        { name: "id", value: 1 },
      ]);
      if (await reader.next()) updated = converter.convert(reader.current!, Item);
    }
    await command.execute("DELETE FROM items WHERE id = $id", [{ name: "id", value: 1 }]);
    let gone: Item | undefined;
    {
      await using reader = await command.executeReader("SELECT id, name FROM items WHERE id = $id", [
        { name: "id", value: 1 },
      ]);
      if (await reader.next()) gone = converter.convert(reader.current!, Item);
    }
    expect(updated?.id).toBe(1);
    expect(updated?.name).toBe("A");
    expect(gone).toBeUndefined();
  });

  it("maps flat contacts", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
    await command.executeNative(new Native(`CREATE TABLE users (
      id INTEGER PRIMARY KEY,
      username TEXT,
      email TEXT,
      contact_name1 TEXT,
      contact_name2 TEXT
    )`));
    await command.execute(
      "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')",
    );
    await using reader = await command.executeReader(
      "SELECT contact_name1, contact_name2, email, id, username FROM users",
    );
    const converter = new RowTypeConverter<User>();
    const mapper = userContactsMapper();
    let user: User | undefined;
    while (await reader.next()) user = mapper.map(converter.convert(reader.current!, User));
    expect(user?.username).toBe("bobby");
    expect(user?.email).toBe("bob@asdf.com");
    expect(user?.contacts.map((contact) => contact.name)).toEqual(["bob", "roberta"]);
  });

  it("reads multiple results returned in a single command", async () => {
    const ds = Datasource.default;
    await using connection = await ds.createConnection();
    await connection.open();
    const command = connection.createCommand();
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
