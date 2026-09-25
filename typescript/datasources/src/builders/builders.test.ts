import { describe, expect, it } from "vitest";
import {
  Aggregate,
  ArrayExpression,
  CheckConstraint,
  ColumnDefault,
  CreateIndexStatement,
  CreateTableStatement,
  DeleteStatement,
  ForeignKey,
  Identifier,
  InsertStatement,
  Join,
  Literal,
  NotNull,
  Operator,
  OrderBy,
  PrimaryKey,
  SelectStatement,
  ReferentialAction,
  StatementEvaluator,
  ParameterDirection,
  ReturnStatement,
  UniqueConstraint,
  ExecuteStoredProcedureStatement,
  UpdateStatement,
  WhereText,
  Where,
  eq,
  ne,
  gt,
  lt,
  ge,
  le,
} from "../index.ts";
import type { ITableConstraint, TableConstraintKind } from "../index.ts";
import type { Parameter } from "../index.ts";

describe("Query and Where", () => {
  it("keeps optional where on the query", () => {
    const select = new SelectStatement("SELECT 1");
    const where = new WhereText("id = 1");
    const withWhere = new SelectStatement("SELECT 1", where);
    expect(select.where).toBeUndefined();
    expect(withWhere.where).toBe(where);
  });

  it("returns constructor text", () => {
    expect(new SelectStatement("SELECT 1").getText()).toBe("SELECT 1");
    expect(new WhereText("id = 1").getText()).toBe("id = 1");
  });
});

describe("Where", () => {
  it("renders compare, groups, and not", () => {
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    expect(where.getText()).toBe("id = $id AND age > $age");
    expect(where.parameters).toEqual([
      { name: "id", value: 1 },
      { name: "age", value: 18 },
    ]);
    expect(new Where(new Operator.NotEquals("id", 2)).getText()).toBe("id <> $id");
    expect(new Where(new Operator.LessThan("n", 0)).getText()).toBe("n < $n");
    expect(new Where(new Operator.GreaterThanOrEqual("n", 0)).getText()).toBe("n >= $n");
    expect(new Where(new Operator.LessThanOrEqual("n", 0)).getText()).toBe("n <= $n");
    expect(new Where(new Operator.Equals(new Literal(1), new Literal(1))).getText()).toBe("1 = 1");
    expect(new Where(new Operator.Equals(new Literal(1), new Literal(0))).getText()).toBe("1 = 0");
    expect(new Where(new Operator.Equals("id", 1)).getText()).toBe("id = $id");
    expect(new Where(new Operator.Or(where, new Operator.Equals("name", "a"))).getText()).toBe(
      "(id = $id AND age > $age) OR name = $name",
    );
    expect(
      new Where(
        new Operator.And(
          new Operator.Or(new Operator.Equals("id", 1), new Operator.Equals("id", 2)),
          new Operator.Equals("age", 18),
        ),
      ).getText(),
    ).toBe("(id = $id OR id = $id) AND age = $age");
    expect(new Where(new Operator.Not(new Where(new Operator.Equals("id", 1)))).getText()).toBe(
      "NOT (id = $id)",
    );
    expect(new Where(new Operator.Not(new Where(new Operator.Equals("id", 1)))).parameters).toEqual([
      { name: "id", value: 1 },
    ]);
    expect(new Where(new Operator.In("id", [])).getText()).toBe("1 = 0");
    expect(new Where(new Operator.In("id", [])).parameters).toEqual([]);
    expect(new Where(new Operator.In("id", [1, 2])).getText()).toBe("id IN ($id_0, $id_1)");
    expect(new Where(new Operator.In("id", [1, 2])).parameters).toEqual([
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
    expect(evaluator.evaluate(new Where(new Operator.Equals("id", 1)), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.Equals("id", 2)), row)).toBe(false);
    expect(evaluator.evaluate(new Where(new Operator.NotEquals("id", 2)), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.GreaterThan("age", 18)), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.LessThan("age", 30)), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.GreaterThanOrEqual("age", 21)), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.LessThanOrEqual("age", 21)), row)).toBe(true);
    expect(
      evaluator.evaluate(
        new Where(new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18))),
        row,
      ),
    ).toBe(true);
    expect(
      evaluator.evaluate(
        new Where(new Operator.Or(new Operator.Equals("id", 0), new Operator.Equals("name", "a"))),
        row,
      ),
    ).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.Not(new Operator.Equals("id", 1))), row)).toBe(false);
    expect(evaluator.evaluate(new Where(new Operator.In("id", [1, 2])), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.Equals(new Literal(1), new Literal(1))), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.Equals(new Literal(1), new Literal(0))), row)).toBe(false);
    expect(() => evaluator.evaluate(new WhereText("id = 1"), row)).toThrow("text where is not evaluable");
    expect(() => evaluator.evaluate(new ArrayExpression([]), row)).toThrow("array is not a scalar");
    expect(() =>
      evaluator.evaluate(new Operator.GreaterThan(new Literal(true), new Literal(1)), row),
    ).toThrow("empty compare");
    expect(() =>
      evaluator.evaluate(new Operator.In(new Identifier("id"), new Literal(1)), row),
    ).toThrow("empty in");
    expect(evaluator.evaluate(new Where(new Operator.LessThan("name", "b")), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.GreaterThanOrEqual("name", "a")), row)).toBe(true);
    expect(evaluator.evaluate(new Where(new Operator.GreaterThan("name", "Z")), row)).toBe(true);
    expect(
      new Where(new Operator.In(new Identifier("id"), new ArrayExpression([]))).getText(),
    ).toBe("1 = 0");
    expect(new Where(new Identifier("id")).parameters).toEqual([]);
    expect(new Where(new ArrayExpression([new Literal(1)])).parameters).toEqual([]);
    expect(() => new Where(new WhereText("x")).parameters).toThrow("empty expression");
    expect(
      evaluator.evaluate(new Operator.GreaterThan(new Literal(2n), new Literal(1n)), row),
    ).toBe(true);
    expect(
      new Where(new Operator.Equals(new Identifier("a"), new Identifier("b"))).getText(),
    ).toBe("a = b");
    expect(
      new Where(new Operator.Equals(new Identifier("a"), new Identifier("b"))).parameters,
    ).toEqual([]);
    expect(new Where(new Operator.In(new Literal(1), new Literal(2))).getText()).toBe("1 IN 2");
    expect(() => new Where(new WhereText("x")).getText()).toThrow("empty expression");
    expect(evaluator.evaluate(new Identifier("missing"), row)).toBeNull();
    expect(evaluator.evaluate(new Literal(null), row)).toBeNull();
    expect(() => evaluator.evaluate(new WhereText("x"), row)).toThrow("text where is not evaluable");
    expect(
      new Where(new Operator.Equals(new Literal(true), new Literal(false))).getText(),
    ).toBe("true = false");
    expect(
      new Where(new Operator.Equals(new Literal(null), new Literal(null))).getText(),
    ).toBe("null = null");
    expect(
      new Where(new Operator.Equals(new Literal(true), new Literal(false))).parameters,
    ).toEqual([]);
    class Ghost extends Operator {}
    expect(() => evaluator.evaluate(new Ghost(), row)).toThrow("empty expression");
    expect(() => new Where(new Ghost()).getText()).toThrow("empty expression");
    expect(() => new Where(new Ghost()).parameters).toThrow("empty expression");
    expect(new Where(new ArrayExpression([new Literal(1)])).getText()).toBe("1");
    expect(
      () =>
        new Where(
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
    const where = new Where(new Operator.Equals("id", 1));
    const query = new SelectStatement({ table: "users", fields: ["id", "name"], where });
    expect(query.getText()).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(query.where).toBe(where);
    expect(new SelectStatement({ table: "users", fields: [] }).getText()).toBe(
      "SELECT * FROM users",
    );
    expect(new SelectStatement({ table: "users", fields: [] }).where).toBeUndefined();
    expect(new SelectStatement({ table: "users", fields: ["id", "name"] }).getText()).toBe(
      "SELECT id, name FROM users",
    );
    expect(
      new SelectStatement({
        table: "users",
        fields: ["users.id", "orders.total"],
        joins: [Join.inner("orders", "users.id", "orders.user_id")],
      }).getText(),
    ).toBe(
      "SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id",
    );
    expect(
      new SelectStatement({
        table: "users",
        fields: ["users.id"],
        where: new Where(new Operator.Equals("users.id", 1)),
        joins: [
          Join.inner("orders", "users.id", "orders.user_id"),
          Join.left("payments", "orders.id", "payments.order_id"),
          Join.right("profiles", "users.id", "profiles.user_id"),
        ],
      }).getText(),
    ).toBe(
      "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
    );
    expect(new SelectStatement({ table: "users", fields: ["id"], joins: [] }).getText()).toBe(
      "SELECT id FROM users",
    );
    const grouped = new SelectStatement({
      table: "orders",
      fields: ["status", Aggregate.count("*", "n"), Aggregate.sum("amount", "total")],
      where: new Where(new Operator.Equals("status", "open")),
      joins: [Join.inner("customers", "orders.customer_id", "customers.id")],
      groupBy: ["status"],
      orderBy: [OrderBy.desc("total"), OrderBy.asc("status")],
      limit: 10,
    });
    expect(grouped.getText()).toBe(
      "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10",
    );
    const byStatus = new SelectStatement({ table: "users", fields: ["id"], groupBy: ["id"] });
    expect(byStatus.getText()).toBe("SELECT id FROM users GROUP BY id");
    const bounds = new SelectStatement({ table: "users", fields: [Aggregate.min("n"), Aggregate.max("n")] });
    expect(bounds.getText()).toBe("SELECT MIN(n), MAX(n) FROM users");
    const ordered = new SelectStatement({
      table: "users",
      fields: ["id"],
      orderBy: [OrderBy.asc("id"), OrderBy.desc("name")],
    });
    expect(ordered.getText()).toBe("SELECT id FROM users ORDER BY id ASC, name DESC");
    const limited = new SelectStatement({ table: "users", fields: ["id"], limit: 10 });
    expect(limited.getText()).toBe("SELECT id FROM users LIMIT 10");
    expect(new SelectStatement({ table: "users", fields: ["id"], limit: 0 }).getText()).toBe(
      "SELECT id FROM users LIMIT 0",
    );
    expect(() => new SelectStatement({ table: "users", fields: ["id"], limit: -1 })).toThrow(
      "negative limit",
    );
    const insert = new InsertStatement("users", [...fields]);
    expect(insert.getText()).toBe(
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
    const createTable = new CreateTableStatement("users", [
      { name: "id", type: "INTEGER" },
      { name: "name", type: "TEXT" },
    ]);
    expect(createTable.getText()).toBe("CREATE TABLE users (id INTEGER, name TEXT)");
    const namedTable = new CreateTableStatement(
      "orders",
      [
        { name: "id", type: "INTEGER", default: new ColumnDefault("0", "df_id"), notNull: new NotNull("nn_id") },
        { name: "tenant_id", type: "INTEGER" },
        { name: "user_id", type: "INTEGER" },
        { name: "qty", type: "INTEGER" },
      ],
      [
        new PrimaryKey("pk_orders", ["id", "tenant_id"]),
        new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade),
        new UniqueConstraint("uq_orders_user", ["user_id"]),
        new CheckConstraint("chk_orders_qty", "qty > 0"),
      ],
    );
    expect(namedTable.getText()).toBe(
      "CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))",
    );
    expect(
      new CreateTableStatement(
        "items",
        [
          { name: "id", type: "INTEGER", default: new ColumnDefault("0") },
          { name: "name", type: "TEXT", notNull: new NotNull() },
        ],
        [new ForeignKey("fk_items_user", ["id"], "users", ["id"])],
      ).getText(),
    ).toBe(
      "CREATE TABLE items (id INTEGER DEFAULT 0, name TEXT NOT NULL, CONSTRAINT fk_items_user FOREIGN KEY (id) REFERENCES users (id))",
    );
    expect(new CreateTableStatement("users", [{ name: "id", type: "INTEGER" }], []).getText()).toBe(
      "CREATE TABLE users (id INTEGER)",
    );
    expect(
      new CreateTableStatement(
        "t",
        [{ name: "id", type: "INTEGER" }],
        [new ForeignKey("fk_set_null", ["id"], "u", ["id"], ReferentialAction.SetNull, ReferentialAction.SetDefault)],
      ).getText(),
    ).toBe(
      "CREATE TABLE t (id INTEGER, CONSTRAINT fk_set_null FOREIGN KEY (id) REFERENCES u (id) ON DELETE SET NULL ON UPDATE SET DEFAULT)",
    );
    expect(
      new CreateTableStatement(
        "t",
        [{ name: "id", type: "INTEGER" }],
        [new ForeignKey("fk_restrict", ["id"], "u", ["id"], ReferentialAction.Restrict, ReferentialAction.NoAction)],
      ).getText(),
    ).toBe(
      "CREATE TABLE t (id INTEGER, CONSTRAINT fk_restrict FOREIGN KEY (id) REFERENCES u (id) ON DELETE RESTRICT ON UPDATE NO ACTION)",
    );
    expect(() =>
      new CreateTableStatement("t", [{ name: "id", type: "INTEGER" }], [
        { kind: "nope" as TableConstraintKind, name: "x" } as ITableConstraint,
      ]).getText(),
    ).toThrow("empty constraint");
    const createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
    expect(createIndex.getText()).toBe("CREATE INDEX idx_orders_user ON orders (user_id)");
    expect(new CreateIndexStatement("uq_orders_code", "orders", ["code"], true).getText()).toBe(
      "CREATE UNIQUE INDEX uq_orders_code ON orders (code)",
    );
    const call = new ExecuteStoredProcedureStatement("save", [...fields]);
    expect(call.getText()).toBe("CALL save($id, $name)");
    expect(new ExecuteStoredProcedureStatement("save", []).getText()).toBe(
      "CALL save()",
    );
    const ret = new ReturnStatement("add", [...fields]);
    expect(ret.getText()).toBe("SELECT add($id, $name)");
    expect(new ReturnStatement("add", []).getText()).toBe("SELECT add()");
    expect(
      new ExecuteStoredProcedureStatement("save", [
        { name: "id", value: 1 },
        { name: "ret", direction: ParameterDirection.ReturnValue },
      ]).getText(),
    ).toBe("CALL save($id)");
    expect(
      new ReturnStatement("add", [
        { name: "id", value: 1 },
        { name: "ret", direction: ParameterDirection.ReturnValue },
      ]).getText(),
    ).toBe("SELECT add($id)");
    const input: Parameter = { name: "id", value: 1 };
    const output: Parameter = { name: "count", direction: ParameterDirection.Output };
    expect(input.direction).toBeUndefined();
    expect(output.direction).toBe(ParameterDirection.Output);
  });

  it("wraps raw statement text", () => {
    expect(new InsertStatement("x").getText()).toBe("x");
    expect(new UpdateStatement("x").getText()).toBe("x");
    expect(DeleteStatement.raw("x").getText()).toBe("x");
    expect(new CreateTableStatement("x").getText()).toBe("x");
    expect(new CreateIndexStatement("x").getText()).toBe("x");
    expect(new ExecuteStoredProcedureStatement("x").getText()).toBe("x");
    expect(new ReturnStatement("x").getText()).toBe("x");
  });
});

describe("comparison aliases", () => {
  it("renders comparison symbols", () => {
    const equals = new Where("username", eq, "grace");
    const notEquals = new Where("id", ne, 2);
    const greater = new Where("n", gt, 0);
    const less = new Where("n", lt, 0);
    const greaterOrEqual = new Where("n", ge, 0);
    const lessOrEqual = new Where("n", le, 0);
    const both = equals.and("is_enabled", eq, 0);
    const either = equals.or("is_enabled", eq, 0);
    expect(equals.getText()).toBe("username = $username");
    expect(notEquals.getText()).toBe("id <> $id");
    expect(greater.getText()).toBe("n > $n");
    expect(less.getText()).toBe("n < $n");
    expect(greaterOrEqual.getText()).toBe("n >= $n");
    expect(lessOrEqual.getText()).toBe("n <= $n");
    expect(both.getText()).toBe("username = $username AND is_enabled = $is_enabled");
    expect(either.getText()).toBe("username = $username OR is_enabled = $is_enabled");
  });
});
