import { describe, expect, it } from "vitest";
import {
  CRUDRepository,
  QueryStatement,
  RowTypeConverter,
  TransformMapper,
  Where,
  WhereClause,
  Operator,
  RepositoryBuilder,
  createRepositoryOptions,
  parts,
  InsertStatement,
  UpdateStatement,
  DeleteStatement,
} from "./index.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";

class Item {
  id = 0;
  name = "";
}

class Label {
  name = "";
}

class PascalId {
  Id = 0;
}

class Composite {
  org_id = 0;
  user_id = 0;
  name = "";
}

const asType = <T>(type: new () => T) => type as Constructable<Cells>;

const create = (overrides: Partial<CRUDRepositoryOptions<Cells>> = {}) => {
  const connection = new MockConnection(undefined, "memory");
  const command = new MockCommand(connection);
  connection.command = command;
  const type = overrides.type ?? asType(Item);
  const options = {
    ...overrides,
    type,
    entity: overrides.entity ?? "users",
    rowConverter: overrides.rowConverter ?? new RowTypeConverter(),
  };
  const repo = new CRUDRepository<Cells>(connection, options);
  return { repo, command };
};

describe("CRUDRepository", () => {
  it("builds select and maps rows", async () => {
    const { repo, command } = create();
    repo.addMiddleware({ beforeCommand() {}, afterCommand() {} });
    command.reader = new MockReader([
      { id: 1, name: "a" },
    ]);
    const rows: Cells[] = [];
    for await (const row of repo.read()) rows.push(row);
    expect(rows).toEqual([{ id: 1, name: "a" }]);
    expect(command.executeReaderCalls[0]?.text).toBe("SELECT id, name FROM users");
    command.reader = new MockReader([{ id: 1, name: "a" }]);
    expect(await repo.readSingle()).toEqual({ id: 1, name: "a" });
  });

  it("uses caller query and returns empty", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([]);
    const where = new WhereClause(new Operator.Equals("id", 9));
    expect(
      await repo.readSingle(new QueryStatement("users", ["id"], where)),
    ).toBeUndefined();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT id FROM users WHERE id = $id",
    );
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);
  });

  it("attaches where to the default select", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([]);
    expect(await repo.readSingle(new WhereClause(new Operator.Equals("id", 9)))).toBeUndefined();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);
  });

  it("adds updates and deletes through builders", async () => {
    const { repo, command } = create();
    command.response = { result: 9, rowsAffected: 1 };
    const obj = { id: 1, name: "a" };
    await repo.create(obj);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO users (name) VALUES ($name)",
    );
    expect(obj.id).toBe(9);
    await repo.update(obj);
    expect(command.executeCalls[1]?.text).toBe(
      "UPDATE users SET name = $name WHERE id = $id",
    );
    await repo.delete(obj);
    expect(command.executeCalls[2]?.text).toBe("DELETE FROM users WHERE id = $id");
  });

  it("maps create result onto Id when id is absent", async () => {
    const { repo, command } = create({ type: asType(PascalId), ids: ["Id"] });
    command.response = { result: 9, rowsAffected: 1 };
    const item = Object.assign(new PascalId(), { Id: 0 });
    await repo.create(item);
    expect(item.Id).toBe(9);
  });

  it("skips apply when create identity is missing", async () => {
    const { repo, command } = create({
      create: new TransformMapper(() => undefined as never, () => null),
    });
    command.response = { result: 9, rowsAffected: 1 };
    const obj = { id: 1, name: "a" };
    await repo.create(obj);
    expect(obj.id).toBe(1);
  });

  it("updates without id columns using a tautology", async () => {
    const { repo, command } = create({ ids: [] });
    await repo.update({ id: 1, name: "a" });
    expect(command.executeCalls[0]?.text).toContain("1 = 1");
    await repo.delete({ id: 1, name: "a" });
    expect(command.executeCalls[1]?.parameters).toEqual([
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ]);
  });

  it("supports composite id and omits ids on create", async () => {
    const { repo, command } = create({
      ids: ["org_id", "user_id"],
      type: asType(Composite),
    });
    await repo.create({ org_id: 1, user_id: 2, name: 3 });
    expect(command.executeCalls[0]?.text).toBe("INSERT INTO users (name) VALUES ($name)");
    expect(command.executeCalls[0]?.parameters?.[0]?.value).toBe(3);
    await repo.update({ org_id: 1, user_id: 2, name: 4 });
    expect(command.executeCalls[1]?.text).toContain(
      "org_id = $org_id AND user_id = $user_id",
    );
  });

  it("throws on missing id, bad rows, and empty update", async () => {
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    connection.command = command;
    const { repo, command: query } = create();
    query.reader = { current: undefined, next: async () => true };
    await expect(async () => {
      for await (const _ of repo.read()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row");
    await expect(repo.update({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.delete({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.update({ id: 1 })).rejects.toThrow("non-Id field");
    query.reader = { current: undefined, next: async () => true };
    await expect(async () => {
      for await (const _ of repo.read()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row");
    await repo.update({ id: 1, name: "a" }, new Where("id = 1"));
    const typed = new CRUDRepository(connection, {
      type: asType(Item),
      entity: "users",
      rowConverter: new RowTypeConverter(),
    });
    await typed.create(Object.assign(new Item(), { id: 1, name: "a" }));
    expect(command.executeCalls[0]?.text).toContain("INSERT INTO users");
    await typed.delete(Object.assign(new Item(), { name: "z" }), new WhereClause(new Operator.Equals("name", "z")));
    expect(command.executeCalls[1]?.text).toBe("DELETE FROM users WHERE name = $name");
  });

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

  it("applies mapper after row converter", async () => {
    const { repo, command } = create({
      rowsMapper: new TransformMapper<Cells, Cells>(
        (row) => ({ ...row, name: String(row.name).toUpperCase() }),
        (row) => ({ ...row, name: String(row.name).toLowerCase() }),
      ),
    });
    command.reader = new MockReader([{ id: 1, name: "a" }]);
    expect(await repo.readSingle()).toEqual({ id: 1, name: "A" });
  });

  it("maps a flat row into nested user contacts", async () => {
    const connection = new MockConnection(undefined, "memory");
    const command = new MockCommand(connection);
    connection.command = command;
    const repo = new CRUDRepository<User>(connection, {
      type: User,
      entity: "users",
      rowConverter: new RowTypeConverter<User>(),
      rowsMapper: userContactsMapper(),
    });
    const flat = {
      id: 1,
      username: "bobby",
      email: "bob@asdf.com",
      contact_name1: "bob",
      contact_name2: "roberta",
    };
    command.reader = new MockReader([flat]);
    const user = await repo.readSingle();
    expect(user?.username).toBe("bobby");
    expect(user?.email).toBe("bob@asdf.com");
    expect(user?.contacts.map((contact) => contact.name)).toEqual(["bob", "roberta"]);
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT contact_name1, contact_name2, email, id, username FROM users",
    );
  });

  it("reverse maps nested user on add update and delete", async () => {
    const connection = new MockConnection(undefined, "memory");
    const command = new MockCommand(connection);
    connection.command = command;
    const repo = new CRUDRepository<User>(connection, {
      type: User,
      entity: "users",
      rowConverter: new RowTypeConverter<User>(),
      rowsMapper: userContactsMapper(),
    });
    const user = Object.assign(new User(), {
      id: 1,
      username: "bobby",
      email: "bob@asdf.com",
      contacts: [
        Object.assign(new Contact(), { name: "bob" }),
        Object.assign(new Contact(), { name: "roberta" }),
      ],
    });
    await repo.create(user);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO users (contact_name1, contact_name2, email, username) VALUES ($contact_name1, $contact_name2, $email, $username)",
    );
    const values = Object.fromEntries(
      (command.executeCalls[0]?.parameters ?? []).map((parameter) => [
        parameter.name,
        parameter.value,
      ]),
    );
    expect(values.contact_name1).toBe("bob");
    expect(values.contact_name2).toBe("roberta");
    await repo.update(user);
    expect(command.executeCalls[1]?.text).not.toContain("contacts");
    await repo.delete(user);
    expect(command.executeCalls[2]?.text).toBe("DELETE FROM users WHERE id = $id");
  });

  it("builds repository options from a type", () => {
    const connection = new MockConnection();
    const fromId = createRepositoryOptions(asType(Item));
    expect(fromId.ids).toEqual(["id"]);
    expect(fromId.entity).toBe("Item");
    expect(createRepositoryOptions(asType(Label)).ids).toEqual([]);
    expect(createRepositoryOptions(asType(PascalId)).ids).toEqual(["Id"]);
    expect(new CRUDRepository(connection, fromId)).toBeInstanceOf(CRUDRepository);
    expect(new CRUDRepository(asType(Item), connection)).toBeInstanceOf(CRUDRepository);
    expect(new CRUDRepository(parts(connection, fromId))).toBeInstanceOf(CRUDRepository);
    expect(RepositoryBuilder.from(connection, fromId).parts().connection).toBe(connection);
    expect(RepositoryBuilder.from(asType(Item), connection).parts().ids).toEqual(["id"]);
    expect(parts(connection, {}).statements.query().getText()).toBe("SELECT * FROM ");
    expect(parts(connection, { type: asType(Item) }).statements.query().getText()).toBe(
      "SELECT id, name FROM Item",
    );
  });

  it("builder maps entity and columns; statement mapper rewrites query", async () => {
    const entity = new TransformMapper<string, string>(
      (name) => (name === "users" ? "people" : name),
      (name) => name,
    );
    const columnMappings = new TransformMapper<string, string>(
      (name) => ({ id: "user_id", name: "full_name" }[name] ?? name),
      (name) => name,
    );
    const { repo, command } = create({
      entityMapper: entity,
      columnMappings,
      query: new TransformMapper(
        (query) => new QueryStatement(`${query.getText()} LIMIT 1`, query.where),
        (query) => query,
      ),
      insert: new TransformMapper(
        (insert) => new InsertStatement(`${insert.getText()} RETURNING id`),
        (insert) => insert,
      ),
      update: new TransformMapper(
        (update) =>
          new UpdateStatement(`${update.getText()} RETURNING id`, update.where),
        (update) => update,
      ),
      delete: new TransformMapper(
        (remove) =>
          DeleteStatement.raw(`${remove.getText()} RETURNING id`, remove.where),
        (remove) => remove,
      ),
    });
    command.reader = new MockReader([]);
    await repo.readSingle();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT full_name, user_id FROM people LIMIT 1",
    );
    const row = { id: 1, name: "a" };
    await repo.create(row);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO people (name) VALUES ($name) RETURNING id",
    );
    await repo.update(row);
    expect(command.executeCalls[1]?.text).toBe(
      "UPDATE people SET name = $name WHERE id = $id RETURNING id",
    );
    await repo.delete(row);
    expect(command.executeCalls[2]?.text).toBe(
      "DELETE FROM people WHERE id = $id RETURNING id",
    );
  });
});
