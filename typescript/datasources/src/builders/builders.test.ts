import { describe, expect, it } from "vitest";
import {
  CreateTableStatement,
  DeleteStatement,
  InsertStatement,
  Query,
  StatementBuilder,
  StoredProcedureStatement,
  UpdateStatement,
  Where,
  WhereBuilder,
} from "../index.ts";

describe("Query and Where", () => {
  it("keeps optional where on the query", () => {
    const where = new Where("id = 1");
    expect(new Query("SELECT 1").where).toBeUndefined();
    expect(new Query("SELECT 1", where).where).toBe(where);
  });
});

describe("WhereBuilder", () => {
  it("renders compare, groups, and not", () => {
    const where = WhereBuilder.and(
      WhereBuilder.equals("id", 1),
      WhereBuilder.greaterThan("age", 18),
    );
    expect(where.getText()).toBe("id = $id AND age > $age");
    expect(where.parameters).toEqual([
      { name: "id", value: 1 },
      { name: "age", value: 18 },
    ]);
    expect(WhereBuilder.ne("id", 2).getText()).toBe("id <> $id");
    expect(WhereBuilder.lt("n", 0).getText()).toBe("n < $n");
    expect(WhereBuilder.gte("n", 0).getText()).toBe("n >= $n");
    expect(WhereBuilder.lte("n", 0).getText()).toBe("n <= $n");
    expect(WhereBuilder.and().getText()).toBe("1 = 1");
    expect(WhereBuilder.or().getText()).toBe("1 = 0");
    expect(WhereBuilder.and(WhereBuilder.equals("id", 1)).getText()).toBe("id = $id");
    expect(WhereBuilder.or(WhereBuilder.equals("id", 1)).getText()).toBe("id = $id");
    expect(WhereBuilder.or(where, WhereBuilder.equals("name", "a")).getText()).toBe(
      "(id = $id AND age > $age) OR name = $name",
    );
    expect(WhereBuilder.not(WhereBuilder.equals("id", 1)).getText()).toBe(
      "NOT (id = $id)",
    );
    expect(WhereBuilder.not(WhereBuilder.equals("id", 1)).parameters).toEqual([
      { name: "id", value: 1 },
    ]);
  });
});

describe("StatementBuilder", () => {
  const fields = [
    { name: "id", value: 1 },
    { name: "name", value: "a" },
  ] as const;

  it("creates write and ddl statements", () => {
    const where = WhereBuilder.equals("id", 1);
    const query = StatementBuilder.createQuery("users", ["id", "name"], where);
    expect(query.getText()).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(query.where).toBe(where);
    expect(StatementBuilder.createQuery("users", []).getText()).toBe(
      "SELECT * FROM users",
    );
    expect(StatementBuilder.createQuery("users", []).where).toBeUndefined();
    expect(StatementBuilder.createQuery("users", ["id", "name"]).getText()).toBe(
      "SELECT id, name FROM users",
    );
    expect(StatementBuilder.createInsert("users", [...fields]).getText()).toBe(
      "INSERT INTO users (id, name) VALUES ($id, $name)",
    );
    const update = StatementBuilder.createUpdate("users", [...fields], where);
    expect(update.getText()).toBe(
      "UPDATE users SET id = $id, name = $name WHERE id = $id",
    );
    expect(update.where).toBe(where);
    expect(StatementBuilder.createUpdate("users", [...fields]).getText()).toBe(
      "UPDATE users SET id = $id, name = $name",
    );
    const del = StatementBuilder.createDelete("users", where);
    expect(del.getText()).toBe("DELETE FROM users WHERE id = $id");
    expect(del.where).toBe(where);
    expect(StatementBuilder.createDelete("users").getText()).toBe(
      "DELETE FROM users",
    );
    expect(
      StatementBuilder.createTable("users", [
        { name: "id", type: "INTEGER" },
        { name: "name", type: "TEXT" },
      ]).getText(),
    ).toBe("CREATE TABLE users (id INTEGER, name TEXT)");
    expect(
      StatementBuilder.createStoredProcedure("save", [...fields]).getText(),
    ).toBe("CALL save($id, $name)");
    expect(StatementBuilder.createStoredProcedure("save").getText()).toBe(
      "CALL save()",
    );
  });

  it("wraps raw statement text", () => {
    expect(new InsertStatement("x").getText()).toBe("x");
    expect(new UpdateStatement("x").getText()).toBe("x");
    expect(new DeleteStatement("x").getText()).toBe("x");
    expect(new CreateTableStatement("x").getText()).toBe("x");
    expect(new StoredProcedureStatement("x").getText()).toBe("x");
  });
});
