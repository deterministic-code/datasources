import { describe, expect, it } from "vitest";
import {
  Repository,
  StatementBuilder,
  RowTypeConverter,
  StatementMapper,
  TransformMapper,
  Where,
  WhereBuilder,
  RepositoryBuilder,
  createRepositoryBuilderOptions,
  parts,
  InsertStatement,
  UpdateStatement,
  DeleteStatement,
} from "./index.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import { Query } from "./Query.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";

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

const create = (overrides: Partial<RepositoryBuilderOptions<Cells>> = {}) => {
  const connection = new MockConnection(undefined, "memory");
  const command = new MockCommand(connection);
  connection.command = command;
  const type = overrides.type ?? asType(Item);
  const repo = new Repository<Cells>({
    ...overrides,
    connection,
    type,
    entity: overrides.entity ?? "users",
    mapping: overrides.mapping ?? new RowTypeConverter(),
  });
  return { repo, command };
};

describe("Repository", () => {
  it("builds select and maps rows", async () => {
    const { repo, command } = create();
    repo.addMiddleware({ beforeQuery() {}, afterQuery() {} });
    command.reader = new MockReader([
      { id: 1, name: "a" },
    ]);
    const rows: Cells[] = [];
    for await (const row of repo.get()) rows.push(row);
    expect(rows).toEqual([{ id: 1, name: "a" }]);
    expect(command.executeReaderCalls[0]?.text).toBe("SELECT id, name FROM users");
    command.reader = new MockReader([{ id: 1, name: "a" }]);
    expect(await repo.getSingle()).toEqual({ id: 1, name: "a" });
  });

  it("uses caller query and returns empty", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([]);
    const where = WhereBuilder.equals("id", 9);
    expect(
      await repo.getSingle(StatementBuilder.createQuery("users", ["id"], where)),
    ).toBeUndefined();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT id FROM users WHERE id = $id",
    );
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);
  });

  it("attaches where to the default select", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([]);
    expect(await repo.getSingle(WhereBuilder.equals("id", 9))).toBeUndefined();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);
  });

  it("adds updates and deletes through builders", async () => {
    const { repo, command } = create({ assignIdentityOnCreate: false });
    const obj = { id: 1, name: "a" };
    await repo.create(obj);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO users (id, name) VALUES ($id, $name)",
    );
    await repo.update(obj);
    expect(command.executeCalls[1]?.text).toBe(
      "UPDATE users SET name = $name WHERE id = $id",
    );
    await repo.delete(obj);
    expect(command.executeCalls[2]?.text).toBe("DELETE FROM users WHERE id = $id");
  });

  it("supports composite id, assignIdentityOnCreate, and overrides", async () => {
    const { repo, command } = create({
      ids: ["org_id", "user_id"],
      assignIdentityOnCreate: true,
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
      for await (const _ of repo.get()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row");
    await expect(repo.update({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.delete({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.update({ id: 1 })).rejects.toThrow("non-Id field");
    query.reader = { current: undefined, next: async () => true };
    await expect(async () => {
      for await (const _ of repo.get()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row");
    await repo.update({ id: 1, name: "a" }, new Where("id = 1"));
    const typed = new Repository({
      connection,
      type: asType(Item),
      entity: "users",
      mapping: new RowTypeConverter(),
    });
    await typed.create(Object.assign(new Item(), { id: 1, name: "a" }));
    expect(command.executeCalls[0]?.text).toContain("INSERT INTO users");
    await typed.delete(Object.assign(new Item(), { name: "z" }), WhereBuilder.equals("name", "z"));
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
      mapper: new TransformMapper<Cells, Cells>(
        (row) => ({ ...row, name: String(row.name).toUpperCase() }),
        (row) => ({ ...row, name: String(row.name).toLowerCase() }),
      ),
    });
    command.reader = new MockReader([{ id: 1, name: "a" }]);
    expect(await repo.getSingle()).toEqual({ id: 1, name: "A" });
  });

  it("maps a flat row into nested user contacts", async () => {
    const connection = new MockConnection(undefined, "memory");
    const command = new MockCommand(connection);
    connection.command = command;
    const repo = new Repository<User>({
      connection,
      type: User,
      entity: "users",
      mapping: new RowTypeConverter<User>(),
      mapper: userContactsMapper(),
    });
    const flat = {
      id: 1,
      username: "bobby",
      email: "bob@asdf.com",
      contact_name1: "bob",
      contact_name2: "roberta",
    };
    command.reader = new MockReader([flat]);
    const user = await repo.getSingle();
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
    const repo = new Repository<User>({
      connection,
      type: User,
      entity: "users",
      assignIdentityOnCreate: false,
      mapping: new RowTypeConverter<User>(),
      mapper: userContactsMapper(),
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
      "INSERT INTO users (contact_name1, contact_name2, email, id, username) VALUES ($contact_name1, $contact_name2, $email, $id, $username)",
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

  it("builds repository options from a type and connection", () => {
    const connection = new MockConnection();
    const fromId = createRepositoryBuilderOptions(asType(Item), connection);
    expect(fromId.ids).toEqual(["id"]);
    expect(fromId.connection).toBe(connection);
    expect(createRepositoryBuilderOptions(asType(Label), connection).ids).toEqual([]);
    expect(createRepositoryBuilderOptions(asType(PascalId), connection).ids).toEqual(["Id"]);
    expect(new Repository(fromId)).toBeInstanceOf(Repository);
    expect(new Repository(asType(Item), connection)).toBeInstanceOf(Repository);
    expect(new Repository(parts(fromId))).toBeInstanceOf(Repository);
    expect(RepositoryBuilder.from(fromId).parts().connection).toBe(connection);
    expect(RepositoryBuilder.from(asType(Item), connection).parts().ids).toEqual(["id"]);
    expect(parts({ connection }).statements.query().getText()).toBe("SELECT * FROM ");
  });

  it("builder maps entity and columns; statement mapper rewrites query", async () => {
    const names = new TransformMapper<string, string>(
      (name) =>
        ({ users: "people", id: "user_id", name: "full_name" }[name] ?? name),
      (name) => name,
    );
    const { repo, command } = create({
      assignIdentityOnCreate: false,
      statementMapper: new StatementMapper({
        entity: names,
        columns: names,
        query: new TransformMapper(
          (query) => new Query(`${query.getText()} LIMIT 1`, query.where),
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
            new DeleteStatement(`${remove.getText()} RETURNING id`, remove.where),
          (remove) => remove,
        ),
      }),
    });
    command.reader = new MockReader([]);
    await repo.getSingle();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT full_name, user_id FROM people LIMIT 1",
    );
    const row = { id: 1, name: "a" };
    await repo.create(row);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO people (id, name) VALUES ($id, $name) RETURNING id",
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
