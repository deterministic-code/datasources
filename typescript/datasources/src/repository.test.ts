import { afterEach, describe, expect, it } from "vitest";
import {
  Repository,
  StatementBuilder,
  TypeConverter,
  Where,
  WhereBuilder,
  RepositoryBuilder,
  createRepositoryBuilderOptions,
  identityProviderTypes,
  parts,
  sqlProviderTypes,
} from "./index.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import type { Cell, Cells } from "./converters/Cell.ts";

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

const schema = { id: "INTEGER", name: "TEXT" };

const create = (
  overrides: Partial<ConstructorParameters<typeof Repository<Cells>>[0]> & {
    schema?: Record<string, string>;
    names?: Record<string, string>;
  } = {},
) => {
  TypeConverter.use("memory", identityProviderTypes());
  const connection = new MockConnection(undefined, "memory");
  const command = new MockCommand(connection);
  connection.command = command;
  const { schema: schemaOverride, converters, names, mapping, ...rest } = overrides;
  const repo = new Repository<Cells>({
    connection,
    mapping:
      mapping ??
      TypeConverter.mapping({
        source: "memory",
        destination: "typescript",
        name: "users",
        schema: schemaOverride ?? schema,
        converters,
        names,
      }),
    ...rest,
  });
  return { repo, command };
};

afterEach(() => TypeConverter.reset());

describe("Repository", () => {
  it("builds select and maps rows", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([
      { id: 1, name: "a" },
    ]);
    const rows: Cells[] = [];
    for await (const row of repo.get()) rows.push(row);
    expect(rows).toEqual([{ id: 1, name: "a" }]);
    expect(command.executeReaderCalls[0]?.text).toBe("SELECT id, name FROM users");
    expect(repo.connection).toBe(command.connection);
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
    const { repo, command } = create({ assignIdentityOnAdd: false });
    const obj = { id: 1, name: "a" };
    await repo.add(obj);
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

  it("supports composite id, assignIdentityOnAdd, and overrides", async () => {
    const twice = {
      to: (value: Cell) => value,
      from: (value: Cell) => (typeof value === "number" ? value * 2 : value),
    };
    const { repo, command } = create({
      ids: ["org_id", "user_id"],
      assignIdentityOnAdd: true,
      schema: { org_id: "INTEGER", user_id: "INTEGER", name: "TEXT" },
      converters: { name: twice },
      names: { name: "label" },
    });
    await repo.add({ org_id: 1, user_id: 2, label: 3 });
    expect(command.executeCalls[0]?.text).toBe("INSERT INTO users (name) VALUES ($name)");
    expect(command.executeCalls[0]?.parameters?.[0]?.value).toBe(6);
    await repo.update({ org_id: 1, user_id: 2, label: 4 });
    expect(command.executeCalls[1]?.text).toContain(
      "org_id = $org_id AND user_id = $user_id",
    );
  });

  it("throws on missing provider, id, bad rows, and empty update", async () => {
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    connection.command = command;
    expect(
      () =>
        new Repository({
          connection,
        }),
    ).toThrow("Provider \"mock\" is not registered");
    const { repo, command: query } = create();
    query.reader = new MockReader(["nope" as never]);
    await expect(async () => {
      for await (const _ of repo.get()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row dictionary");
    await expect(repo.update({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.delete({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.update({ id: 1 })).rejects.toThrow("non-Id field");
    query.reader = new MockReader([[] as never]);
    await expect(async () => {
      for await (const _ of repo.get()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row dictionary");
    await repo.update({ id: 1, name: "a" }, new Where("id = 1"));
    TypeConverter.use("memory", sqlProviderTypes());
    const mapping = TypeConverter.mapping<Item>({
      source: "memory",
      destination: "typescript",
      type: Item,
      name: "users",
    });
    const typed = new Repository<Item>({
      connection,
      mapping,
    });
    await typed.add(Object.assign(new Item(), { id: 1, name: "a" }));
    expect(command.executeCalls[0]?.text).toContain("INSERT INTO users");
    await typed.delete(Object.assign(new Item(), { name: "z" }), WhereBuilder.equals("name", "z"));
    expect(command.executeCalls[1]?.text).toBe("DELETE FROM users WHERE name = $name");
  });

  it("builds repository options from a type and connection", () => {
    TypeConverter.use("mock", identityProviderTypes());
    const connection = new MockConnection();
    const fromId = createRepositoryBuilderOptions(Item, connection);
    expect(fromId.ids).toEqual(["id"]);
    expect(fromId.connection).toBe(connection);
    expect(createRepositoryBuilderOptions(Label, connection).ids).toEqual([]);
    expect(createRepositoryBuilderOptions(PascalId, connection).ids).toEqual(["Id"]);
    const repo = new Repository(fromId);
    expect(repo.connection).toBe(connection);
    expect(new Repository(Item, connection).connection).toBe(connection);
    expect(new Repository(parts(fromId)).connection).toBe(connection);
    expect(RepositoryBuilder.from(fromId).parts().commands.get.connection).toBe(connection);
    expect(RepositoryBuilder.from(Item, connection).parts().ids).toEqual(["id"]);
  });
});
