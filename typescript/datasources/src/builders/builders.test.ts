import { describe, expect, it } from "vitest";
import {
  Aggregate,
  ArrayExpression,
  CheckConstraint,
  ColumnDefault,
  AlterTableStatement,
  CreateIndexStatement,
  CreateTableStatement,
  DropIndexStatement,
  DropTriggerStatement,
  CreateTableStatementTextRenderer,
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
  ExpressionEvaluator,
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
  DefaultStatementTextRenderer,
} from "../index.ts";
import type { ITableConstraint, TableConstraintKind } from "../index.ts";
import type { Parameter } from "../index.ts";

describe("Query and Where", () => {
  it("keeps optional where on the query", () => {
    const select = new SelectStatement({ table: "dual", fields: ["1"] });
    const where = new WhereText("id = 1");
    const withWhere = new SelectStatement({ table: "dual", fields: ["1"], where });
    expect(DefaultStatementTextRenderer.instance.render(select)).toBe("SELECT 1 FROM dual");
    expect(select.where).toBeUndefined();
    expect(withWhere.where).toBe(where);
    expect(DefaultStatementTextRenderer.instance.render(where)).toBe("id = 1");
  });
});

describe("Where", () => {
  it("renders compare, groups, and not", () => {
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    expect(DefaultStatementTextRenderer.instance.render(where)).toBe("id = $id AND age > $age");
    expect(where.parameters).toEqual([
      { name: "id", value: 1 },
      { name: "age", value: 18 },
    ]);
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.NotEquals("id", 2)))).toBe("id <> $id");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.LessThan("n", 0)))).toBe("n < $n");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.GreaterThanOrEqual("n", 0)))).toBe("n >= $n");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.LessThanOrEqual("n", 0)))).toBe("n <= $n");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.Equals(new Literal(1), new Literal(1))))).toBe("1 = 1");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.Equals(new Literal(1), new Literal(0))))).toBe("1 = 0");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.Equals("id", 1)))).toBe("id = $id");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.Or(where, new Operator.Equals("name", "a"))))).toBe(
      "(id = $id AND age > $age) OR name = $name",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(new Where(
        new Operator.And(
          new Operator.Or(new Operator.Equals("id", 1), new Operator.Equals("id", 2)),
          new Operator.Equals("age", 18),
        ),
      )),
    ).toBe("(id = $id OR id = $id) AND age = $age");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.Not(new Where(new Operator.Equals("id", 1)))))).toBe(
      "NOT (id = $id)",
    );
    expect(new Where(new Operator.Not(new Where(new Operator.Equals("id", 1)))).parameters).toEqual([
      { name: "id", value: 1 },
    ]);
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.In("id", [])))).toBe("1 = 0");
    expect(new Where(new Operator.In("id", [])).parameters).toEqual([]);
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.In("id", [1, 2])))).toBe("id IN ($id_0, $id_1)");
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

describe("ExpressionEvaluator", () => {
  it("evaluates compare, groups, not, and in", () => {
    expect(new StatementEvaluator().evaluate(new SelectStatement({ table: "dual", fields: ["1"] }))).toBeUndefined();
    const evaluator = new ExpressionEvaluator();
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
      DefaultStatementTextRenderer.instance.render(new Where(new Operator.In(new Identifier("id"), new ArrayExpression([])))),
    ).toBe("1 = 0");
    expect(new Where(new Identifier("id")).parameters).toEqual([]);
    expect(new Where(new ArrayExpression([new Literal(1)])).parameters).toEqual([]);
    expect(() => new Where(new WhereText("x")).parameters).toThrow("empty expression");
    expect(
      evaluator.evaluate(new Operator.GreaterThan(new Literal(2n), new Literal(1n)), row),
    ).toBe(true);
    expect(
      DefaultStatementTextRenderer.instance.render(new Where(new Operator.Equals(new Identifier("a"), new Identifier("b")))),
    ).toBe("a = b");
    expect(
      new Where(new Operator.Equals(new Identifier("a"), new Identifier("b"))).parameters,
    ).toEqual([]);
    expect(DefaultStatementTextRenderer.instance.render(new Where(new Operator.In(new Literal(1), new Literal(2))))).toBe("1 IN 2");
    expect(() => DefaultStatementTextRenderer.instance.render(new Where(new WhereText("x")))).toThrow("empty expression");
    expect(evaluator.evaluate(new Identifier("missing"), row)).toBeNull();
    expect(evaluator.evaluate(new Literal(null), row)).toBeNull();
    expect(() => evaluator.evaluate(new WhereText("x"), row)).toThrow("text where is not evaluable");
    expect(
      DefaultStatementTextRenderer.instance.render(new Where(new Operator.Equals(new Literal(true), new Literal(false)))),
    ).toBe("true = false");
    expect(
      DefaultStatementTextRenderer.instance.render(new Where(new Operator.Equals(new Literal(null), new Literal(null)))),
    ).toBe("null = null");
    expect(
      new Where(new Operator.Equals(new Literal(true), new Literal(false))).parameters,
    ).toEqual([]);
    class Ghost extends Operator {}
    expect(() => evaluator.evaluate(new Ghost(), row)).toThrow("empty expression");
    expect(() => DefaultStatementTextRenderer.instance.render(new Where(new Ghost()))).toThrow("empty expression");
    expect(() => new Where(new Ghost()).parameters).toThrow("empty expression");
    expect(DefaultStatementTextRenderer.instance.render(new Where(new ArrayExpression([new Literal(1)])))).toBe("1");
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
    expect(DefaultStatementTextRenderer.instance.render(query)).toBe(
      "SELECT id, name FROM users WHERE id = $id",
    );
    expect(query.where).toBe(where);
    expect(DefaultStatementTextRenderer.instance.render(new SelectStatement({ table: "users", fields: [] }))).toBe(
      "SELECT * FROM users",
    );
    expect(new SelectStatement({ table: "users", fields: [] }).where).toBeUndefined();
    expect(DefaultStatementTextRenderer.instance.render(new SelectStatement({ table: "users", fields: ["id", "name"] }))).toBe(
      "SELECT id, name FROM users",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(new SelectStatement({
        table: "users",
        fields: ["users.id", "orders.total"],
        joins: [Join.inner("orders", "users.id", "orders.user_id")],
      })),
    ).toBe(
      "SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(new SelectStatement({
        table: "users",
        fields: ["users.id"],
        where: new Where(new Operator.Equals("users.id", 1)),
        joins: [
          Join.inner("orders", "users.id", "orders.user_id"),
          Join.left("payments", "orders.id", "payments.order_id"),
          Join.right("profiles", "users.id", "profiles.user_id"),
        ],
      })),
    ).toBe(
      "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
    );
    expect(DefaultStatementTextRenderer.instance.render(new SelectStatement({ table: "users", fields: ["id"], joins: [] }))).toBe(
      "SELECT id FROM users",
    );
    const grouped = new SelectStatement({
      table: "orders",
      fields: ["status", Aggregate.count("*"), Aggregate.sum("amount")],
      aliases: [undefined, "n", "total"],
      where: new Where(new Operator.Equals("status", "open")),
      joins: [Join.inner("customers", "orders.customer_id", "customers.id")],
      groupBy: ["status"],
      orderBy: [OrderBy.desc("total"), OrderBy.asc("status")],
      limit: 10,
    });
    expect(DefaultStatementTextRenderer.instance.render(grouped)).toBe(
      "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10",
    );
    const byStatus = new SelectStatement({ table: "users", fields: ["id"], groupBy: ["id"] });
    expect(DefaultStatementTextRenderer.instance.render(byStatus)).toBe("SELECT id FROM users GROUP BY id");
    const bounds = new SelectStatement({ table: "users", fields: [Aggregate.min("n"), Aggregate.max("n")] });
    expect(DefaultStatementTextRenderer.instance.render(bounds)).toBe("SELECT MIN(n), MAX(n) FROM users");
    const ordered = new SelectStatement({
      table: "users",
      fields: ["id"],
      orderBy: [OrderBy.asc("id"), OrderBy.desc("name")],
    });
    expect(DefaultStatementTextRenderer.instance.render(ordered)).toBe("SELECT id FROM users ORDER BY id ASC, name DESC");
    const limited = new SelectStatement({ table: "users", fields: ["id"], limit: 10 });
    expect(DefaultStatementTextRenderer.instance.render(limited)).toBe("SELECT id FROM users LIMIT 10");
    expect(DefaultStatementTextRenderer.instance.render(new SelectStatement({ table: "users", fields: ["id"], limit: 0 }))).toBe(
      "SELECT id FROM users LIMIT 0",
    );
    expect(() =>
      DefaultStatementTextRenderer.instance.render(
        new SelectStatement({ table: "users", fields: ["id"], limit: -1 }),
      ),
    ).toThrow("negative limit");
    const insert = new InsertStatement("users", [...fields]);
    expect(DefaultStatementTextRenderer.instance.render(insert)).toBe(
      "INSERT INTO users (id, name) VALUES ($id, $name)",
    );
    const update = new UpdateStatement("users", [...fields], where);
    expect(DefaultStatementTextRenderer.instance.render(update)).toBe(
      "UPDATE users SET id = $id, name = $name WHERE id = $id",
    );
    expect(update.where).toBe(where);
    expect(DefaultStatementTextRenderer.instance.render(new UpdateStatement("users", [...fields]))).toBe(
      "UPDATE users SET id = $id, name = $name",
    );
    const del = new DeleteStatement("users", where);
    expect(DefaultStatementTextRenderer.instance.render(del)).toBe("DELETE FROM users WHERE id = $id");
    expect(del.where).toBe(where);
    expect(DefaultStatementTextRenderer.instance.render(new DeleteStatement("users"))).toBe(
      "DELETE FROM users",
    );
    const createTable = new CreateTableStatement("users", [
      { name: "id", type: "INTEGER" },
      { name: "name", type: "TEXT" },
    ]);
    expect(DefaultStatementTextRenderer.instance.render(createTable)).toBe("CREATE TABLE users (id INTEGER, name TEXT)");
    expect(new class extends CreateTableStatementTextRenderer {
      override render(_statement: CreateTableStatement): string {
        return "CREATE TABLE replaced";
      }
    }().render(createTable)).toBe("CREATE TABLE replaced");
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
    expect(DefaultStatementTextRenderer.instance.render(namedTable)).toBe(
      "CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(new CreateTableStatement(
        "items",
        [
          { name: "id", type: "INTEGER", default: new ColumnDefault("0") },
          { name: "name", type: "TEXT", notNull: new NotNull() },
        ],
        [new ForeignKey("fk_items_user", ["id"], "users", ["id"])],
      )),
    ).toBe(
      "CREATE TABLE items (id INTEGER DEFAULT 0, name TEXT NOT NULL, CONSTRAINT fk_items_user FOREIGN KEY (id) REFERENCES users (id))",
    );
    expect(DefaultStatementTextRenderer.instance.render(new CreateTableStatement("users", [{ name: "id", type: "INTEGER" }], []))).toBe(
      "CREATE TABLE users (id INTEGER)",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(new CreateTableStatement(
        "t",
        [{ name: "id", type: "INTEGER" }],
        [new ForeignKey("fk_set_null", ["id"], "u", ["id"], ReferentialAction.SetNull, ReferentialAction.SetDefault)],
      )),
    ).toBe(
      "CREATE TABLE t (id INTEGER, CONSTRAINT fk_set_null FOREIGN KEY (id) REFERENCES u (id) ON DELETE SET NULL ON UPDATE SET DEFAULT)",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(new CreateTableStatement(
        "t",
        [{ name: "id", type: "INTEGER" }],
        [new ForeignKey("fk_restrict", ["id"], "u", ["id"], ReferentialAction.Restrict, ReferentialAction.NoAction)],
      )),
    ).toBe(
      "CREATE TABLE t (id INTEGER, CONSTRAINT fk_restrict FOREIGN KEY (id) REFERENCES u (id) ON DELETE RESTRICT ON UPDATE NO ACTION)",
    );
    expect(() =>
      DefaultStatementTextRenderer.instance.render(new CreateTableStatement("t", [{ name: "id", type: "INTEGER" }], [
        { kind: "nope" as TableConstraintKind, name: "x" } as ITableConstraint,
      ])),
    ).toThrow("empty constraint");
    const createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
    expect(DefaultStatementTextRenderer.instance.render(createIndex)).toBe("CREATE INDEX idx_orders_user ON orders (user_id)");
    expect(DefaultStatementTextRenderer.instance.render(new CreateIndexStatement("uq_orders_code", "orders", ["code"], true))).toBe(
      "CREATE UNIQUE INDEX uq_orders_code ON orders (code)",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(
        new AlterTableStatement("users", { kind: "addColumn", column: { name: "email", type: "TEXT", notNull: new NotNull() } }),
      ),
    ).toBe("ALTER TABLE users ADD COLUMN email TEXT NOT NULL");
    expect(
      DefaultStatementTextRenderer.instance.render(new AlterTableStatement("users", { kind: "dropColumn", name: "email" })),
    ).toBe("ALTER TABLE users DROP COLUMN email");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new AlterTableStatement("users", {
          kind: "alterColumn",
          column: { name: "age", type: "INTEGER", default: new ColumnDefault("0"), notNull: new NotNull() },
        }),
      ),
    ).toBe("ALTER TABLE users ALTER COLUMN age TYPE INTEGER DEFAULT 0 NOT NULL");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new AlterTableStatement("users", { kind: "renameColumn", from: "email", to: "email_address" }),
      ),
    ).toBe("ALTER TABLE users RENAME COLUMN email TO email_address");
    expect(
      DefaultStatementTextRenderer.instance.render(new AlterTableStatement("users", { kind: "renameTable", to: "accounts" })),
    ).toBe("ALTER TABLE users RENAME TO accounts");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new AlterTableStatement("orders", { kind: "addConstraint", constraint: new UniqueConstraint("uq_orders_user", ["user_id"]) }),
      ),
    ).toBe("ALTER TABLE orders ADD CONSTRAINT uq_orders_user UNIQUE (user_id)");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new AlterTableStatement("orders", { kind: "dropConstraint", name: "uq_orders_user" }),
      ),
    ).toBe("ALTER TABLE orders DROP CONSTRAINT uq_orders_user");
    expect(DefaultStatementTextRenderer.instance.render(new DropIndexStatement("idx_orders_user", "orders"))).toBe(
      "DROP INDEX IF EXISTS idx_orders_user",
    );
    expect(DefaultStatementTextRenderer.instance.render(new DropTriggerStatement("trg_users_updated", "users"))).toBe(
      "DROP TRIGGER IF EXISTS trg_users_updated",
    );
    const call = new ExecuteStoredProcedureStatement("save", [...fields]);
    expect(DefaultStatementTextRenderer.instance.render(call)).toBe("CALL save($id, $name)");
    expect(DefaultStatementTextRenderer.instance.render(new ExecuteStoredProcedureStatement("save", []))).toBe(
      "CALL save()",
    );
    const ret = new ReturnStatement("add", [...fields]);
    expect(DefaultStatementTextRenderer.instance.render(ret)).toBe("SELECT add($id, $name)");
    expect(DefaultStatementTextRenderer.instance.render(new ReturnStatement("add", []))).toBe("SELECT add()");
    expect(
      DefaultStatementTextRenderer.instance.render(new ExecuteStoredProcedureStatement("save", [
        { name: "id", value: 1 },
        { name: "ret", direction: ParameterDirection.ReturnValue },
      ])),
    ).toBe("CALL save($id)");
    expect(
      DefaultStatementTextRenderer.instance.render(new ReturnStatement("add", [
        { name: "id", value: 1 },
        { name: "ret", direction: ParameterDirection.ReturnValue },
      ])),
    ).toBe("SELECT add($id)");
    const input: Parameter = { name: "id", value: 1 };
    const output: Parameter = { name: "count", direction: ParameterDirection.Output };
    expect(input.direction).toBeUndefined();
    expect(output.direction).toBe(ParameterDirection.Output);
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
    expect(DefaultStatementTextRenderer.instance.render(equals)).toBe("username = $username");
    expect(DefaultStatementTextRenderer.instance.render(notEquals)).toBe("id <> $id");
    expect(DefaultStatementTextRenderer.instance.render(greater)).toBe("n > $n");
    expect(DefaultStatementTextRenderer.instance.render(less)).toBe("n < $n");
    expect(DefaultStatementTextRenderer.instance.render(greaterOrEqual)).toBe("n >= $n");
    expect(DefaultStatementTextRenderer.instance.render(lessOrEqual)).toBe("n <= $n");
    expect(DefaultStatementTextRenderer.instance.render(both)).toBe("username = $username AND is_enabled = $is_enabled");
    expect(DefaultStatementTextRenderer.instance.render(either)).toBe("username = $username OR is_enabled = $is_enabled");
  });
});
