import { afterEach, describe, expect, it } from "vitest";
import {
  Repository,
  StatementBuilder,
  TypeConverter,
  Where,
  WhereBuilder,
  identityProviderTypes,
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

const schema = { id: "INTEGER", name: "TEXT" };

const create = (overrides: Partial<ConstructorParameters<typeof Repository<Cells>>[0]> = {}) => {
  TypeConverter.use("memory", identityProviderTypes());
  const connection = new MockConnection();
  const command = new MockCommand(connection);
  const repo = new Repository<Cells>({
    datasourceName: "users",
    connection,
    createCommand: () => command,
    provider: "memory",
    schema,
    ...overrides,
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

  it("adds updates and deletes through builders", async () => {
    const { repo, command } = create();
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

  it("supports composite id, generated insert, and overrides", async () => {
    const twice = {
      to: (value: Cell) => value,
      from: (value: Cell) => (typeof value === "number" ? value * 2 : value),
    };
    const { repo, command } = create({
      id: ["org_id", "user_id"],
      generated: true,
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
    const command = new MockCommand(new MockConnection());
    expect(
      () =>
        new Repository({
          datasourceName: "users",
          connection: command.connection,
          createCommand: () => command,
        }),
    ).toThrow("Repository requires Provider or Mapping");
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
    TypeConverter.use("memory", sqlProviderTypes({}));
    const mapping = TypeConverter.mapping<Item>({
      source: "memory",
      destination: "typescript",
      type: Item,
    });
    const typed = new Repository<Item>({
      datasourceName: "users",
      connection: command.connection,
      createCommand: () => command,
      mapping,
      type: Item,
    });
    await typed.add(Object.assign(new Item(), { id: 1, name: "a" }));
    expect(command.executeCalls[0]?.text).toContain("INSERT INTO users");
    await typed.delete(Object.assign(new Item(), { name: "z" }), WhereBuilder.equals("name", "z"));
    expect(command.executeCalls[1]?.text).toBe("DELETE FROM users WHERE name = $name");
  });
});
