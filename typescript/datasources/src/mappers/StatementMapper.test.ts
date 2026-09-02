import { describe, expect, it } from "vitest";
import { DeleteStatement } from "../builders/DeleteStatement.ts";
import { InsertStatement } from "../builders/InsertStatement.ts";
import { UpdateStatement } from "../builders/UpdateStatement.ts";
import { Query } from "../Query.ts";
import { statementsOf } from "../RepositoryStatements.ts";
import { TransformMapper } from "./TransformMapper.ts";
import { StatementMapper } from "./StatementMapper.ts";

const names = new TransformMapper<string, string>(
  (name) =>
    ({ users: "people", id: "user_id", name: "full_name" }[name] ?? name),
  (name) => name,
);

describe("StatementMapper", () => {
  it("defaults are identity", () => {
    const mapper = new StatementMapper<string>();
    expect(mapper.map("x")).toBe("x");
    expect(mapper.reverseMap("x")).toBe("x");
    expect(mapper.entity.map("id")).toBe("id");
    expect(mapper.columns.reverseMap("id")).toBe("id");
  });

  it("rewrites each finished statement", () => {
    const mapper = new StatementMapper({
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

  it("delegates row map", () => {
    const mapper = new StatementMapper({
      rows: new TransformMapper(
        (row: { name: string }) => ({ name: row.name.toUpperCase() }),
        (row) => row,
      ),
    });
    expect(mapper.map({ name: "a" }).name).toBe("A");
    expect(mapper.reverseMap({ name: "A" }).name).toBe("A");
  });
});
