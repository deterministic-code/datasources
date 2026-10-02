import { describe, expect, it } from "vitest";
import {
  CRUDRepository,
  DefaultStatementTextRenderer,
  ExecuteStoredProcedureStatement,
  SelectStatement,
  MetaDataRowTypeConverter,
  TransformMapper,
  WhereText,
  Where,
  Operator,
  CRUDRepositoryBuilder,
  ReadonlyRepositoryBuilder,
  createCRUDRepositoryOptions,
} from "./index.ts";
import { MockCommand } from "./mocks/mock-command.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import { MockReader } from "./mocks/mock-reader.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import { TypeConverter } from "./converters/TypeConverter.ts";
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
    objectConverter: overrides.objectConverter ?? new MetaDataRowTypeConverter(),
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
    const where = new Where(new Operator.Equals("id", 9));
    expect(
      await repo.readSingle(new SelectStatement({ table: "users", fields: ["id"], where })),
    ).toBeUndefined();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT id FROM users WHERE id = $id",
    );
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);
  });

  it("attaches where to the default select", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([]);
    expect(await repo.readSingle(new Where(new Operator.Equals("id", 9)))).toBeUndefined();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);
  });

  it("calls a stored procedure and maps rows", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([{ id: 1, name: "a" }]);
    const parameters = [{ name: "id", value: 9 }];
    const call = new ExecuteStoredProcedureStatement("get_user", parameters);
    const rows: Cells[] = [];
    for await (const row of repo.read(call, parameters)) rows.push(row);
    expect(rows).toEqual([{ id: 1, name: "a" }]);
    expect(command.executeReaderCalls[0]?.text).toBe("CALL get_user($id)");
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);
    command.reader = new MockReader([{ id: 1, name: "a" }]);
    expect(await repo.readSingle(call, parameters)).toEqual({ id: 1, name: "a" });
  });

  it("adds updates and deletes through builders", async () => {
    const { repo, command } = create();
    command.response = { result: undefined, rowsAffected: 1 };
    command.reader = new MockReader([{ id: 9 }]);
    const obj = { id: 1, name: "a" };
    const created = await repo.create(obj);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO users (name) VALUES ($name)",
    );
    expect(command.executeReaderCalls[0]?.text).toBe("users.id");
    expect(created.id).toBe(9);
    const updated = await repo.update(obj);
    expect(updated.rowsAffected).toBe(1);
    expect(command.executeCalls[1]?.text).toBe(
      "UPDATE users SET name = $name WHERE id = $id",
    );
    const removed = await repo.delete(obj);
    expect(removed.rowsAffected).toBe(1);
    expect(command.executeCalls[2]?.text).toBe("DELETE FROM users WHERE id = $id");
  });

  it("maps create result onto Id when id is absent", async () => {
    const { repo, command } = create({ type: asType(PascalId), ids: ["Id"] });
    command.response = { result: undefined, rowsAffected: 1 };
    command.reader = new MockReader([{ Id: 9 }]);
    const item = Object.assign(new PascalId(), { Id: 0 });
    const created = await repo.create(item);
    expect(created.Id).toBe(9);
  });

  it("updates without id columns using a tautology", async () => {
    const { repo, command } = create({ ids: [] });
    command.response = { result: undefined, rowsAffected: 1 };
    const updated = await repo.update({ id: 1, name: "a" });
    expect(updated.rowsAffected).toBe(1);
    expect(command.executeCalls[0]?.text).toContain("1 = 1");
    await expect(repo.readSingleById(1)).rejects.toThrow("ReadSingleById requires an Id column");
    const removed = await repo.delete({ id: 1, name: "a" });
    expect(removed.rowsAffected).toBe(1);
    expect(command.executeCalls[1]?.parameters).toEqual([]);
    const removedByWhere = await repo.delete({ id: 1, name: "a" }, new WhereText("id = $id"));
    expect(removedByWhere.rowsAffected).toBe(1);
    expect(command.executeCalls[2]?.parameters).toEqual([
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ]);
    const versioned = create();
    versioned.command.response = { result: undefined, rowsAffected: 1 };
    const versionedUpdate = await versioned.repo.update(
      { id: 1, name: "b", name_extra: "x" },
      new Where(new Operator.And(
        new Operator.Equals("name_extra", "x"),
        new Operator.Equals("name", "a"),
      )),
    );
    expect(versionedUpdate.rowsAffected).toBe(1);
    expect(versioned.command.executeCalls[0]?.text).toContain("name = $name__where");
    expect(versioned.command.executeCalls[0]?.text).toContain("name_extra = $name_extra");
    const versionedWhere = await versioned.repo.update({ id: 1, name: "b" }, {
      accept: () => "name = $name",
      parameters: [{ name: "skipped" }, { name: "name", value: "a" }],
    });
    expect(versionedWhere.rowsAffected).toBe(1);
    expect(versioned.command.executeCalls[1]?.text).toContain("name = $name__where");
  });

  it("supports composite id and omits ids on create", async () => {
    const { repo, command } = create({
      ids: ["org_id", "user_id"],
      type: asType(Composite),
    });
    const created = await repo.create({ org_id: 1, user_id: 2, name: 3 });
    expect(created.org_id).toBe(1);
    expect(created.user_id).toBe(2);
    expect(command.executeCalls[0]?.text).toBe("INSERT INTO users (name) VALUES ($name)");
    expect(command.executeCalls[0]?.parameters?.[0]?.value).toBe(3);
    command.response = { result: undefined, rowsAffected: 1 };
    const updated = await repo.update({ org_id: 1, user_id: 2, name: 4 });
    expect(updated.rowsAffected).toBe(1);
    expect(command.executeCalls[1]?.text).toContain(
      "org_id = $org_id AND user_id = $user_id",
    );
  });

  it("omits null datetime and version on create and update", async () => {
    const { repo, command } = create();
    command.response = { result: undefined, rowsAffected: 1 };
    command.reader = new MockReader([{ id: 7 }, { id: 7 }]);
    const created = await repo.create({ id: 1, name: "ada", created: null, version: undefined });
    expect(created.id).toBe(7);
    expect(command.executeCalls[0]?.text).toBe("INSERT INTO users (name) VALUES ($name)");
    const updated = await repo.update({ id: 1, name: "ada", created: null, version: null });
    expect(updated.rowsAffected).toBe(1);
    expect(command.executeCalls[1]?.text).toBe(
      "UPDATE users SET name = $name WHERE id = $id",
    );
    const when = new Date("2024-06-01T00:00:00.000Z");
    const grace = await repo.create({ id: 1, name: "grace", created: when, version: new Uint8Array([9, 9]) });
    expect(grace.id).toBe(7);
    expect(command.executeCalls[2]?.text).toBe(
      "INSERT INTO users (created, name, version) VALUES ($created, $name, $version)",
    );
  });

  it("reads a single row by id columns", async () => {
    const { repo, command } = create();
    command.reader = new MockReader([]);
    expect(await repo.readSingleById(9)).toBeUndefined();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(command.executeReaderCalls[0]?.parameters).toEqual([{ name: "id", value: 9 }]);

    const composite = create({ ids: ["org_id", "user_id"], type: asType(Composite) });
    composite.command.reader = new MockReader([]);
    expect(await composite.repo.readSingleById({ org_id: 1, user_id: 2 })).toBeUndefined();
    expect(composite.command.executeReaderCalls[0]?.text).toContain(
      "org_id = $org_id AND user_id = $user_id",
    );
    await expect(composite.repo.readSingleById(1)).rejects.toThrow(
      "Composite id requires a row keyed by each Id column",
    );
    await expect(composite.repo.readSingleById({ org_id: 1 })).rejects.toThrow(
      "Missing Id value",
    );
  });

  it("throws on missing id, bad rows, and empty update", async () => {
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    connection.command = command;
    const { repo, command: query } = create();
    query.reader = {
      current: undefined,
      next: async () => true,
      nextResult: async () => false,
      async *read() {
        while (await this.next()) {
          if (this.current !== undefined) yield this.current;
        }
      },
      dispose: async () => {},
    };
    await expect(async () => {
      for await (const _ of repo.read()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row");
    await expect(repo.update({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.delete({ name: "a" })).rejects.toThrow("Missing Id value");
    await expect(repo.update({ id: 1 })).rejects.toThrow("non-Id field");
    query.reader = {
      current: undefined,
      next: async () => true,
      nextResult: async () => false,
      async *read() {
        while (await this.next()) {
          if (this.current !== undefined) yield this.current;
        }
      },
      dispose: async () => {},
    };
    await expect(async () => {
      for await (const _ of repo.read()) {
        /* consume */
      }
    }).rejects.toThrow("Reader current must be a row");
    query.response = { result: undefined, rowsAffected: 1 };
    const updated = await repo.update({ id: 1, name: "a" }, new WhereText("id = 1"));
    expect(updated.rowsAffected).toBe(1);
    const typed = new CRUDRepository(connection, {
      type: asType(Item),
      entity: "users",
      objectConverter: new MetaDataRowTypeConverter(),
    });
    command.response = { result: undefined, rowsAffected: 1 };
    const created = await typed.create(Object.assign(new Item(), { id: 1, name: "a" }));
    expect(created.id).toBe(1);
    expect(command.executeCalls[0]?.text).toContain("INSERT INTO users");
    const removed = await typed.delete(Object.assign(new Item(), { name: "z" }), new Where(new Operator.Equals("name", "z")));
    expect(removed.rowsAffected).toBe(1);
    expect(command.executeCalls[1]?.text).toBe("DELETE FROM users WHERE name = $name");
  });

  it("builds repository options from a type", () => {
    const connection = new MockConnection();
    const fromId = createCRUDRepositoryOptions(asType(Item));
    expect(fromId.ids).toEqual(["id"]);
    expect(fromId.entity).toBe("Item");
    expect(createCRUDRepositoryOptions(asType(Label)).ids).toEqual([]);
    expect(createCRUDRepositoryOptions(asType(PascalId)).ids).toEqual(["Id"]);
    expect(new CRUDRepository(connection, fromId)).toBeInstanceOf(CRUDRepository);
    expect(new CRUDRepository(asType(Item), connection)).toBeInstanceOf(CRUDRepository);
    expect(new CRUDRepositoryBuilder().parts(connection, fromId).connection).toBe(connection);
    expect(new CRUDRepositoryBuilder().parts(connection, createCRUDRepositoryOptions(asType(Item))).ids).toEqual(["id"]);
    expect(DefaultStatementTextRenderer.instance.render(new ReadonlyRepositoryBuilder().parts(connection, {}).statements.select())).toBe("SELECT * FROM ");
    expect(DefaultStatementTextRenderer.instance.render(new ReadonlyRepositoryBuilder().parts(connection, { type: asType(Item) }).statements.select())).toBe(
      "SELECT id, name FROM Item",
    );
  });

  it("builder maps entity and columns", async () => {
    const entity = new TransformMapper<string, string>(
      (name) => (name === "users" ? "people" : name),
      (name) => name,
    );
    const propertyMappings = new TransformMapper<string, string>(
      (name) => ({ id: "user_id", name: "full_name" }[name] ?? name),
      (name) => name,
    );
    const { repo, command } = create({
      entityMapper: entity,
      propertyMappings,
    });
    command.reader = new MockReader([]);
    await repo.readSingle();
    expect(command.executeReaderCalls[0]?.text).toBe(
      "SELECT full_name, user_id FROM people",
    );
    command.response = { result: undefined, rowsAffected: 1 };
    const row = { id: 1, name: "a" };
    const created = await repo.create(row);
    expect(created.id).toBe(1);
    expect(command.executeCalls[0]?.text).toBe(
      "INSERT INTO people (full_name) VALUES ($full_name)",
    );
    const updated = await repo.update(row);
    expect(updated.rowsAffected).toBe(1);
    expect(command.executeCalls[1]?.text).toBe(
      "UPDATE people SET full_name = $full_name WHERE user_id = $user_id",
    );
    const removed = await repo.delete(row);
    expect(removed.rowsAffected).toBe(1);
    expect(command.executeCalls[2]?.text).toBe(
      "DELETE FROM people WHERE user_id = $user_id",
    );
  });

  it("reverse-converts a field only before values are applied", async () => {
    class Flag extends TypeConverter<Cell, Cell> {
      convert = (value: Cell): Cell => value;
      reverseConvert = (value: Cell): Cell => (value === true ? 1 : 0);
    }
    class Row {
      id = 0;
      on = false;
    }
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    connection.command = command;
    const repo = new CRUDRepository<Row>(connection, {
      type: Row,
      entity: "rows",
      objectConverter: new MetaDataRowTypeConverter<Row>({ on: new Flag() }),
    });
    command.response = { result: undefined, rowsAffected: 1 };
    const created = await repo.create(Object.assign(new Row(), { id: 1, on: true }));
    expect(created.id).toBe(1);
    const updated = await repo.update(
      Object.assign(new Row(), { id: 1, on: true }),
      new Where(new Operator.Equals("on", true)),
    );
    expect(updated.rowsAffected).toBe(1);
    expect(command.executeCalls[1]?.parameters).toContainEqual({ name: "on", value: 1 });
  });

  it("keeps a where value when the converter has no fields", async () => {
    class Plain extends TypeConverter<Item, Cells> {
      convert = (row: Cells): Item => Object.assign(new Item(), row);
      reverseConvert = (row: Item): Cells => ({ id: row.id, name: row.name });
    }
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    connection.command = command;
    const repo = new CRUDRepository<Item>(connection, {
      type: Item,
      entity: "items",
      objectConverter: new Plain(),
    });
    command.response = { result: undefined, rowsAffected: 1 };
    const updated = await repo.update(
      Object.assign(new Item(), { id: 1, name: "a" }),
      new Where(new Operator.Equals("name", "a")),
    );
    expect(updated.rowsAffected).toBe(1);
    expect(command.executeCalls[0]?.parameters).toContainEqual({ name: "name", value: "a" });
  });
});
