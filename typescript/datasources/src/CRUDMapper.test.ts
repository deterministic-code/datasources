import { describe, expect, it } from "vitest";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { ReturnStatement } from "./statements/ReturnStatement.ts";
import { QueryStatement } from "./statements/QueryStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import { statementsOf } from "./RepositoryStatements.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import { CRUDMapper } from "./CRUDMapper.ts";

const names = new TransformMapper<string, string>(
  (name) =>
    ({ users: "people", id: "user_id", name: "full_name" }[name] ?? name),
  (name) => name,
);

describe("CRUDMapper", () => {
  it("defaults are identity", () => {
    const mapper = new CRUDMapper<string>();
    expect(mapper.rows.map("x")).toBe("x");
    expect(mapper.rows.reverseMap("x")).toBe("x");
    expect(mapper.entity.map("id")).toBe("id");
    expect(mapper.entity.reverseMap("id")).toBe("id");
    expect(mapper.columns.map("id")).toBe("id");
    expect(mapper.columns.reverseMap("id")).toBe("id");
  });

  it("rewrites each finished statement", () => {
    const mapper = new CRUDMapper({
      entity: names,
      columns: names,
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
    const statements = statementsOf("users", ["id", "name"], mapper);
    expect(statements.query().getText()).toBe("SELECT id, name FROM users LIMIT 1");
    expect(statements.insert([{ name: "id", value: 1 }]).getText()).toBe(
      "INSERT INTO users (id) VALUES ($id) RETURNING id",
    );
    expect(statements.update([{ name: "name", value: "a" }]).getText()).toBe(
      "UPDATE users SET name = $name RETURNING id",
    );
    expect(statements.delete().getText()).toBe("DELETE FROM users RETURNING id");
  });

  it("rewrites a finished return call", () => {
    const mapper = new CRUDMapper({
      return: new TransformMapper(
        (ret) => new ReturnStatement(`${ret.getText()} /* mapped */`),
        (ret) => ret,
      ),
    });
    const built = new ReturnStatement("add", [{ name: "id", value: 1 }]);
    const statement = mapper.return?.map(built) ?? built;
    expect(statement.getText()).toBe("SELECT add($id) /* mapped */");
  });

  it("maps rows through rows", () => {
    const mapper = new CRUDMapper({
      rows: new TransformMapper(
        (row: { name: string }) => ({ name: row.name.toUpperCase() }),
        (row) => row,
      ),
    });
    expect(mapper.rows.map({ name: "a" }).name).toBe("A");
    expect(mapper.rows.reverseMap({ name: "A" }).name).toBe("A");
  });
});
