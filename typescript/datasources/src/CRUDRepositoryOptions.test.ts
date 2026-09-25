import { describe, expect, it } from "vitest";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { ReturnStatement } from "./statements/ReturnStatement.ts";
import { QueryStatement } from "./statements/QueryStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import { statementsOf } from "./RepositoryStatements.ts";
import { parts } from "./RepositoryBuilder.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";

describe("CRUDRepositoryOptions", () => {
  it("defaults are identity", () => {
    const built = parts(new MockConnection(), { ids: ["id"], entity: "users", readProperties: ["id"] });
    expect(built.statements.query().getText()).toBe("SELECT id FROM users");
    expect(built.rowsMapper.map("x")).toBe("x");
    expect(built.rowsMapper.reverseMap("x")).toBe("x");
    expect(built.create?.map(null)).toEqual({});
    expect(built.create?.map(9)).toEqual({ id: 9 });
    expect(parts(new MockConnection(), { insert: undefined }).ids).toEqual(["id"]);
    expect(
      parts(new MockConnection(), {
        type: class {
          id = 0;
        },
        update: undefined,
      }).ids,
    ).toEqual(["id"]);
  });

  it("rewrites each finished statement", () => {
    const statements = statementsOf(
      "users",
      ["id", "name"],
      new TransformMapper(
        (query) => new QueryStatement(`${query.getText()} LIMIT 1`, query.where),
        (query) => query,
      ),
      new TransformMapper(
        (insert) => new InsertStatement(`${insert.getText()} RETURNING id`),
        (insert) => insert,
      ),
      new TransformMapper(
        (update) =>
          new UpdateStatement(`${update.getText()} RETURNING id`, update.where, update.parameters),
        (update) => update,
      ),
      new TransformMapper(
        (remove) => DeleteStatement.raw(`${remove.getText()} RETURNING id`, remove.where),
        (remove) => remove,
      ),
    );
    expect(statements.query().getText()).toBe("SELECT id, name FROM users LIMIT 1");
    expect(statements.insert([{ name: "id", value: 1 }]).getText()).toBe(
      "INSERT INTO users (id) VALUES ($id) RETURNING id",
    );
    expect(statements.update([{ name: "name", value: "a" }]).getText()).toBe(
      "UPDATE users SET name = $name RETURNING id",
    );
    expect(statements.update([{ name: "name", value: "a" }]).parameters).toEqual([
      { name: "name", value: "a" },
    ]);
    expect(statements.delete().getText()).toBe("DELETE FROM users RETURNING id");
  });

  it("rewrites a finished return call", () => {
    const options: CRUDRepositoryOptions = {
      return: new TransformMapper(
        (ret) => new ReturnStatement(`${ret.getText()} /* mapped */`),
        (ret) => ret,
      ),
    };
    const built = new ReturnStatement("add", [{ name: "id", value: 1 }]);
    const statement = options.return?.map(built) ?? built;
    expect(statement.getText()).toBe("SELECT add($id) /* mapped */");
  });

  it("maps rows through rowsMapper", () => {
    const options: CRUDRepositoryOptions<{ name: string }> = {
      rowsMapper: new TransformMapper(
        (row) => ({ name: row.name.toUpperCase() }),
        (row) => row,
      ),
    };
    expect(options.rowsMapper?.map({ name: "a" }).name).toBe("A");
    expect(options.rowsMapper?.reverseMap({ name: "A" }).name).toBe("A");
  });
});
