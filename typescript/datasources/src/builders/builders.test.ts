import { describe, expect, it } from "vitest";
import {
  ArrayExpression,
  CreateTableStatement,
  DeleteStatement,
  Identifier,
  InsertStatement,
  Join,
  Literal,
  Operator,
  QueryStatement,
  StatementEvaluator,
  StoredProcedureStatement,
  UpdateStatement,
  Where,
  WhereClause,
} from "../index.ts";

describe("Query and Where", () => {
  it("keeps optional where on the query", () => {
    const select = new QueryStatement("SELECT 1");
    const where = new Where("id = 1");
    const withWhere = new QueryStatement("SELECT 1", where);
    expect(select.where).toBeUndefined();
    expect(withWhere.where).toBe(where);
  });
});

describe("WhereClause", () => {
  it("renders compare, groups, and not", () => {
    const where = new WhereClause(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    expect(where.getText()).toBe("id = $id AND age > $age");
    expect(where.parameters).toEqual([
      { name: "id", value: 1 },
      { name: "age", value: 18 },
    ]);
    expect(new WhereClause(new Operator.NotEquals("id", 2)).getText()).toBe("id <> $id");
    expect(new WhereClause(new Operator.LessThan("n", 0)).getText()).toBe("n < $n");
    expect(new WhereClause(new Operator.GreaterThanOrEqual("n", 0)).getText()).toBe("n >= $n");
    expect(new WhereClause(new Operator.LessThanOrEqual("n", 0)).getText()).toBe("n <= $n");
    expect(new WhereClause(new Operator.Equals(new Literal(1), new Literal(1))).getText()).toBe("1 = 1");
    expect(new WhereClause(new Operator.Equals(new Literal(1), new Literal(0))).getText()).toBe("1 = 0");
    expect(new WhereClause(new Operator.Equals("id", 1)).getText()).toBe("id = $id");
    expect(new WhereClause(new Operator.Or(where, new Operator.Equals("name", "a"))).getText()).toBe(
      "(id = $id AND age > $age) OR name = $name",
    );
    expect(
      new WhereClause(
        new Operator.And(
          new Operator.Or(new Operator.Equals("id", 1), new Operator.Equals("id", 2)),
          new Operator.Equals("age", 18),
        ),
      ).getText(),
    ).toBe("(id = $id OR id = $id) AND age = $age");
    expect(new WhereClause(new Operator.Not(new WhereClause(new Operator.Equals("id", 1)))).getText()).toBe(
      "NOT (id = $id)",
    );
    expect(new WhereClause(new Operator.Not(new WhereClause(new Operator.Equals("id", 1)))).parameters).toEqual([
      { name: "id", value: 1 },
    ]);
    expect(new WhereClause(new Operator.In("id", [])).getText()).toBe("1 = 0");
    expect(new WhereClause(new Operator.In("id", [])).parameters).toEqual([]);
    expect(new WhereClause(new Operator.In("id", [1, 2])).getText()).toBe("id IN ($id_0, $id_1)");
    expect(new WhereClause(new Operator.In("id", [1, 2])).parameters).toEqual([
      { name: "id_0", value: 1 },
      { name: "id_1", value: 2 },
    ]);
    const and = where.condition;
    expect(and).toBeInstanceOf(Operator.And);
    if (!(and instanceof Operator.And)) throw new Error("and");
    expect(and.left).toBeInstanceOf(Operator.Equals);
    if (!(and.left instanceof Operator.Equals)) throw new Error("left");
    expect(and.left.left).toBeInstanceOf(Identifier);
    if (!(and.left.left instanceof Identifier)) throw new Error("id");
    expect(and.left.left.name).toBe("id");
    expect(and.left.right).toBeInstanceOf(Literal);
    if (!(and.left.right instanceof Literal)) throw new Error("lit");
    expect(and.left.right.value).toBe(1);
  });
});

describe("StatementEvaluator", () => {
  it("evaluates compare, groups, not, and in", () => {
    const evaluator = new StatementEvaluator();
    const row = { id: 1, age: 21, name: "a" };
    expect(evaluator.evaluate(new WhereClause(new Operator.Equals("id", 1)), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.Equals("id", 2)), row)).toBe(false);
    expect(evaluator.evaluate(new WhereClause(new Operator.NotEquals("id", 2)), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.GreaterThan("age", 18)), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.LessThan("age", 30)), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.GreaterThanOrEqual("age", 21)), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.LessThanOrEqual("age", 21)), row)).toBe(true);
    expect(
      evaluator.evaluate(
        new WhereClause(new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18))),
        row,
      ),
    ).toBe(true);
    expect(
      evaluator.evaluate(
        new WhereClause(new Operator.Or(new Operator.Equals("id", 0), new Operator.Equals("name", "a"))),
        row,
      ),
    ).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.Not(new Operator.Equals("id", 1))), row)).toBe(false);
    expect(evaluator.evaluate(new WhereClause(new Operator.In("id", [1, 2])), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.Equals(new Literal(1), new Literal(1))), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.Equals(new Literal(1), new Literal(0))), row)).toBe(false);
    expect(() => evaluator.evaluate(new Where("id = 1"), row)).toThrow("text where is not evaluable");
    expect(() => evaluator.evaluate(new ArrayExpression([]), row)).toThrow("array is not a scalar");
    expect(() =>
      evaluator.evaluate(new Operator.GreaterThan(new Literal(true), new Literal(1)), row),
    ).toThrow("empty compare");
    expect(() =>
      evaluator.evaluate(new Operator.In(new Identifier("id"), new Literal(1)), row),
    ).toThrow("empty in");
    expect(evaluator.evaluate(new WhereClause(new Operator.LessThan("name", "b")), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.GreaterThanOrEqual("name", "a")), row)).toBe(true);
    expect(evaluator.evaluate(new WhereClause(new Operator.GreaterThan("name", "Z")), row)).toBe(true);
    expect(
      new WhereClause(new Operator.In(new Identifier("id"), new ArrayExpression([]))).getText(),
    ).toBe("1 = 0");
    expect(new WhereClause(new Identifier("id")).parameters).toEqual([]);
    expect(new WhereClause(new ArrayExpression([new Literal(1)])).parameters).toEqual([]);
    expect(() => new WhereClause(new Where("x")).parameters).toThrow("empty expression");
    expect(
      evaluator.evaluate(new Operator.GreaterThan(new Literal(2n), new Literal(1n)), row),
    ).toBe(true);
    expect(
      new WhereClause(new Operator.Equals(new Identifier("a"), new Identifier("b"))).getText(),
    ).toBe("a = b");
    expect(
      new WhereClause(new Operator.Equals(new Identifier("a"), new Identifier("b"))).parameters,
    ).toEqual([]);
    expect(new WhereClause(new Operator.In(new Literal(1), new Literal(2))).getText()).toBe("1 IN 2");
    expect(() => new WhereClause(new Where("x")).getText()).toThrow("empty expression");
    expect(evaluator.evaluate(new Identifier("missing"), row)).toBeNull();
    expect(evaluator.evaluate(new Literal(null), row)).toBeNull();
    expect(() => evaluator.evaluate(new Where("x"), row)).toThrow("text where is not evaluable");
    expect(
      new WhereClause(new Operator.Equals(new Literal(true), new Literal(false))).getText(),
    ).toBe("true = false");
    expect(
      new WhereClause(new Operator.Equals(new Literal(null), new Literal(null))).getText(),
    ).toBe("null = null");
    expect(
      new WhereClause(new Operator.Equals(new Literal(true), new Literal(false))).parameters,
    ).toEqual([]);
    class Ghost extends Operator {}
    expect(() => evaluator.evaluate(new Ghost(), row)).toThrow("empty expression");
    expect(() => new WhereClause(new Ghost()).getText()).toThrow("empty expression");
    expect(() => new WhereClause(new Ghost()).parameters).toThrow("empty expression");
    expect(new WhereClause(new ArrayExpression([new Literal(1)])).getText()).toBe("1");
    expect(
      () =>
        new WhereClause(
          new Operator.In(new Identifier("id"), new ArrayExpression([new Identifier("x")])),
        ).parameters,
    ).toThrow("empty in");
  });
});

describe("statements", () => {
  const fields = [
    { name: "id", value: 1 },
    { name: "name", value: "a" },
  ] as const;

  it("creates write and ddl statements", () => {
    const where = new WhereClause(new Operator.Equals("id", 1));
    const query = new QueryStatement("users", ["id", "name"], where);
    expect(query.getText()).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(query.where).toBe(where);
    expect(new QueryStatement("users", []).getText()).toBe(
      "SELECT * FROM users",
    );
    expect(new QueryStatement("users", []).where).toBeUndefined();
    expect(new QueryStatement("users", ["id", "name"]).getText()).toBe(
      "SELECT id, name FROM users",
    );
    expect(
      new QueryStatement("users", ["users.id", "orders.total"], undefined, [
        Join.inner("orders", "users.id", "orders.user_id"),
      ]).getText(),
    ).toBe(
      "SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id",
    );
    expect(
      new QueryStatement(
        "users",
        ["users.id"],
        new WhereClause(new Operator.Equals("users.id", 1)),
        [
          Join.inner("orders", "users.id", "orders.user_id"),
          Join.left("payments", "orders.id", "payments.order_id"),
          Join.right("profiles", "users.id", "profiles.user_id"),
        ],
      ).getText(),
    ).toBe(
      "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
    );
    expect(new QueryStatement("users", ["id"], undefined, []).getText()).toBe(
      "SELECT id FROM users",
    );
    expect(new InsertStatement("users", [...fields]).getText()).toBe(
      "INSERT INTO users (id, name) VALUES ($id, $name)",
    );
    const update = new UpdateStatement("users", [...fields], where);
    expect(update.getText()).toBe(
      "UPDATE users SET id = $id, name = $name WHERE id = $id",
    );
    expect(update.where).toBe(where);
    expect(new UpdateStatement("users", [...fields]).getText()).toBe(
      "UPDATE users SET id = $id, name = $name",
    );
    const del = new DeleteStatement("users", where);
    expect(del.getText()).toBe("DELETE FROM users WHERE id = $id");
    expect(del.where).toBe(where);
    expect(new DeleteStatement("users").getText()).toBe(
      "DELETE FROM users",
    );
    expect(
      new CreateTableStatement("users", [
        { name: "id", type: "INTEGER" },
        { name: "name", type: "TEXT" },
      ]).getText(),
    ).toBe("CREATE TABLE users (id INTEGER, name TEXT)");
    expect(
      new StoredProcedureStatement("save", [...fields]).getText(),
    ).toBe("CALL save($id, $name)");
    expect(new StoredProcedureStatement("save", []).getText()).toBe(
      "CALL save()",
    );
  });

  it("wraps raw statement text", () => {
    expect(new InsertStatement("x").getText()).toBe("x");
    expect(new UpdateStatement("x").getText()).toBe("x");
    expect(DeleteStatement.raw("x").getText()).toBe("x");
    expect(new CreateTableStatement("x").getText()).toBe("x");
    expect(new StoredProcedureStatement("x").getText()).toBe("x");
  });
});
