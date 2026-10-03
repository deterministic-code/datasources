import { describe, expect, it } from "vitest";
import {
  Aggregate,
  CheckConstraint,
  ColumnDefault,
  CreateIndexStatement,
  CreateTableStatement,
  DeleteStatement,
  ForeignKey,
  InsertStatement,
  Join,
  NotNull,
  Operator,
  OrderBy,
  PrimaryKey,
  SelectStatement,
  ReferentialAction,
  ReturnStatement,
  UniqueConstraint,
  ExecuteStoredProcedureStatement,
  UpdateStatement,
  WhereText,
  Where,
  DefaultStatementTextRenderer,
} from "../index.ts";

describe("statement compile", () => {
  it("wraps query text and an optional where", () => {
    const select = new SelectStatement({ table: "dual", fields: ["1"] });
    const byId = new WhereText("id = 1");
    const withWhere = new SelectStatement({ table: "dual", fields: ["1"], where: byId });
    expect(DefaultStatementTextRenderer.instance.render(select)).toBe("SELECT 1 FROM dual");
    expect(select.where).toBeUndefined();
    expect(withWhere.where).toBe(byId);
    expect(DefaultStatementTextRenderer.instance.render(byId)).toBe("id = 1");
  });

  it("compiles a select", () => {
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    const query = new SelectStatement({ table: "users", fields: ["id", "name"], where });
    expect(DefaultStatementTextRenderer.instance.render(query)).toBe(
      "SELECT id, name FROM users WHERE id = $id AND age > $age",
    );
    expect(query.where).toBe(where);
  });

  it("joins tables", () => {
    const query = new SelectStatement({
      table: "users",
      fields: ["users.id"],
      where: new Where(new Operator.Equals("users.id", 1)),
      joins: [
        Join.inner("orders", "users.id", "orders.user_id"),
        Join.left("payments", "orders.id", "payments.order_id"),
        Join.right("profiles", "users.id", "profiles.user_id"),
      ],
    });
    expect(DefaultStatementTextRenderer.instance.render(query)).toBe(
      "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
    );
  });

  it("groups aggregates orders and limits", () => {
    const grouped = new SelectStatement({
      table: "orders",
      fields: ["status", Aggregate.count("*", "n"), Aggregate.sum("amount", "total")],
      where: new Where(new Operator.Equals("status", "open")),
      joins: [Join.inner("customers", "orders.customer_id", "customers.id")],
      groupBy: ["status"],
      orderBy: [OrderBy.desc("total"), OrderBy.asc("status")],
      limit: 10,
    });
    expect(DefaultStatementTextRenderer.instance.render(grouped)).toBe(
      "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10",
    );
  });

  it("groups rows", () => {
    const byStatus = new SelectStatement({ table: "users", fields: ["id"], groupBy: ["id"] });
    expect(DefaultStatementTextRenderer.instance.render(byStatus)).toBe("SELECT id FROM users GROUP BY id");
  });

  it("aggregates without grouping", () => {
    const bounds = new SelectStatement({ table: "users", fields: [Aggregate.min("n"), Aggregate.max("n")] });
    expect(DefaultStatementTextRenderer.instance.render(bounds)).toBe("SELECT MIN(n), MAX(n) FROM users");
  });

  it("orders rows", () => {
    const ordered = new SelectStatement({
      table: "users",
      fields: ["id"],
      orderBy: [OrderBy.asc("id"), OrderBy.desc("name")],
    });
    expect(DefaultStatementTextRenderer.instance.render(ordered)).toBe(
      "SELECT id FROM users ORDER BY id ASC, name DESC",
    );
  });

  it("limits rows", () => {
    const limited = new SelectStatement({ table: "users", fields: ["id"], limit: 10 });
    expect(DefaultStatementTextRenderer.instance.render(limited)).toBe("SELECT id FROM users LIMIT 10");
  });

  it("compiles an insert", () => {
    const fields = [
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ];
    const insert = new InsertStatement("users", fields);
    expect(DefaultStatementTextRenderer.instance.render(insert)).toBe(
      "INSERT INTO users (id, name) VALUES ($id, $name)",
    );
  });

  it("compiles an update", () => {
    const fields = [
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ];
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    const update = new UpdateStatement("users", fields, where);
    expect(DefaultStatementTextRenderer.instance.render(update)).toBe(
      "UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age",
    );
    expect(update.where).toBe(where);
  });

  it("compiles a delete", () => {
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    const del = new DeleteStatement("users", where);
    expect(DefaultStatementTextRenderer.instance.render(del)).toBe(
      "DELETE FROM users WHERE id = $id AND age > $age",
    );
    expect(del.where).toBe(where);
  });

  it("compiles a create table", () => {
    const createTable = new CreateTableStatement("users", [
      { name: "id", type: "INTEGER" },
      { name: "name", type: "TEXT" },
    ]);
    expect(DefaultStatementTextRenderer.instance.render(createTable)).toBe(
      "CREATE TABLE users (id INTEGER, name TEXT)",
    );
  });

  it("compiles named constraints", () => {
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
  });

  it("compiles a create index", () => {
    const createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
    expect(DefaultStatementTextRenderer.instance.render(createIndex)).toBe(
      "CREATE INDEX idx_orders_user ON orders (user_id)",
    );
  });

  it("compiles a create unique index", () => {
    const createIndex = new CreateIndexStatement("uq_orders_code", "orders", ["code"], true);
    expect(DefaultStatementTextRenderer.instance.render(createIndex)).toBe(
      "CREATE UNIQUE INDEX uq_orders_code ON orders (code)",
    );
  });

  it("compiles a call", () => {
    const fields = [
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ];
    const call = new ExecuteStoredProcedureStatement("save", fields);
    expect(DefaultStatementTextRenderer.instance.render(call)).toBe("CALL save($id, $name)");
  });

  it("compiles a select function call", () => {
    const fields = [
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ];
    const ret = new ReturnStatement("add", fields);
    expect(DefaultStatementTextRenderer.instance.render(ret)).toBe("SELECT add($id, $name)");
  });

  it("renders compare groups not and in", () => {
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    const inIds = new Where(new Operator.In("id", [1, 2]));
    expect(DefaultStatementTextRenderer.instance.render(where)).toBe("id = $id AND age > $age");
    expect(DefaultStatementTextRenderer.instance.render(inIds)).toBe("id IN ($id_0, $id_1)");
  });

  it("compare tree", () => {
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    expect(DefaultStatementTextRenderer.instance.render(where)).toBe("id = $id AND age > $age");
  });

  it("attaches where to a query", () => {
    const where = new Where(
      new Operator.And(new Operator.Equals("id", 1), new Operator.GreaterThan("age", 18)),
    );
    const query = new SelectStatement({ table: "users", fields: ["id", "name"], where });
    expect(DefaultStatementTextRenderer.instance.render(query)).toBe(
      "SELECT id, name FROM users WHERE id = $id AND age > $age",
    );
  });
});
