import { describe, expect, it } from "vitest";
import {
  Aggregate,
  CheckConstraint,
  ColumnDefault,
  AlterTableStatement,
  BulkDeleteStatement,
  BulkUpdateStatement,
  CreateIndexStatement,
  CreateBulkCreateStoredProcedureStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateBulkUpdateStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateDeleteStoredProcedureStatement,
  CreateReadStoredProcedureStatement,
  CreateUpdateStoredProcedureStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  ConditionRenderer,
  DropIndexStatement,
  DropRoutineStatement,
  DropTableStatement,
  DropTriggerStatement,
  ExpressionRenderer,
  Identifier,
  IdentityInsertStatement,
  StatementRenderer,
  StatementTextRenderer,
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

  it("compiles an alter table add column", () => {
    const alter = new AlterTableStatement("users", {
      kind: "addColumn",
      column: { name: "email", type: "TEXT", notNull: new NotNull() },
    });
    expect(DefaultStatementTextRenderer.instance.render(alter)).toBe(
      "ALTER TABLE users ADD COLUMN email TEXT NOT NULL",
    );
  });

  it("compiles an alter table drop column", () => {
    const alter = new AlterTableStatement("users", { kind: "dropColumn", name: "email" });
    expect(DefaultStatementTextRenderer.instance.render(alter)).toBe("ALTER TABLE users DROP COLUMN email");
  });

  it("compiles an alter table alter column", () => {
    const alter = new AlterTableStatement("users", {
      kind: "alterColumn",
      column: { name: "age", type: "INTEGER", default: new ColumnDefault("0"), notNull: new NotNull() },
    });
    expect(DefaultStatementTextRenderer.instance.render(alter)).toBe(
      "ALTER TABLE users ALTER COLUMN age TYPE INTEGER DEFAULT 0 NOT NULL",
    );
  });

  it("compiles an alter table rename column", () => {
    const alter = new AlterTableStatement("users", { kind: "renameColumn", from: "email", to: "email_address" });
    expect(DefaultStatementTextRenderer.instance.render(alter)).toBe(
      "ALTER TABLE users RENAME COLUMN email TO email_address",
    );
  });

  it("compiles an alter table rename table", () => {
    const alter = new AlterTableStatement("users", { kind: "renameTable", to: "accounts" });
    expect(DefaultStatementTextRenderer.instance.render(alter)).toBe("ALTER TABLE users RENAME TO accounts");
  });

  it("compiles an alter table add constraint", () => {
    const alter = new AlterTableStatement("orders", {
      kind: "addConstraint",
      constraint: new UniqueConstraint("uq_orders_user", ["user_id"]),
    });
    expect(DefaultStatementTextRenderer.instance.render(alter)).toBe(
      "ALTER TABLE orders ADD CONSTRAINT uq_orders_user UNIQUE (user_id)",
    );
  });

  it("compiles an alter table drop constraint", () => {
    const alter = new AlterTableStatement("orders", { kind: "dropConstraint", name: "uq_orders_user" });
    expect(DefaultStatementTextRenderer.instance.render(alter)).toBe(
      "ALTER TABLE orders DROP CONSTRAINT uq_orders_user",
    );
  });

  it("compiles a drop index", () => {
    const drop = new DropIndexStatement("idx_orders_user", "orders");
    expect(DefaultStatementTextRenderer.instance.render(drop)).toBe("DROP INDEX IF EXISTS idx_orders_user");
  });

  it("compiles a drop trigger", () => {
    const drop = new DropTriggerStatement("trg_users_updated", "users");
    expect(DefaultStatementTextRenderer.instance.render(drop)).toBe("DROP TRIGGER IF EXISTS trg_users_updated");
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

  it("compiles trigger, routine, identity insert, and bulk update", () => {
    expect(DefaultStatementTextRenderer.instance.render(new DropTableStatement("users"))).toBe(
      "DROP TABLE IF EXISTS users",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateTriggerStatement("users", "tr_users", "updated", "id", "CURRENT_TIMESTAMP"),
      ),
    ).toBe("");
    expect(DefaultStatementTextRenderer.instance.render(new DropRoutineStatement("sp_users"))).toBe("");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateCreateStoredProcedureStatement("CreateUsers", "users", ["id"], [], "id", "INT", []),
      ),
    ).toBe("");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateReadStoredProcedureStatement("ReadUsers", "users", ["id"], [], "id", "INT", [], []),
      ),
    ).toBe("");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateUpdateStoredProcedureStatement("UpdateUsersById", "users", ["id"], [], "id", "INT", [], ["id"]),
      ),
    ).toBe("");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateDeleteStoredProcedureStatement("DeleteUsersById", "users", ["id"], [], "id", "INT", [], ["id"]),
      ),
    ).toBe("");
    const payload = { name: "payload", type: "JSON" };
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateBulkCreateStoredProcedureStatement("BulkCreateUsers", "users", [payload], payload, "id", "INT"),
      ),
    ).toBe("");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateBulkUpdateStoredProcedureStatement("BulkUpdateUsers", "users", [payload], payload, "id", "INT"),
      ),
    ).toBe("");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new CreateBulkDeleteStoredProcedureStatement("BulkDeleteUsers", "users", [payload], payload, "id", "INT"),
      ),
    ).toBe("");
    const insert = new InsertStatement("users", [{ name: "id", value: 1 }]);
    expect(
      DefaultStatementTextRenderer.instance.render(new IdentityInsertStatement("users", "id", [insert], true)),
    ).toBe("INSERT INTO users (id) VALUES (1);");
    const update = new BulkUpdateStatement("users", ["name"], ["id", "org"], [
      { fields: [{ name: "name", value: null }], ids: [{ name: "id", value: 1 }] },
    ]);
    expect(DefaultStatementTextRenderer.instance.render(update)).toBe(
      "UPDATE users SET name = data.name FROM (SELECT $name__0 AS name, $id__0 AS id, $org__0 AS org) AS data WHERE users.id = data.id AND users.org = data.org",
    );
    expect(update.parameters.map((field) => field.name)).toEqual(["name__0", "id__0", "org__0"]);
    expect(
      DefaultStatementTextRenderer.instance.render(new BulkDeleteStatement("users", [], [])),
    ).toBe("DELETE FROM users WHERE 1 = 1");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new BulkDeleteStatement("users", ["id"], [[{ name: "id", value: 1 }]]),
      ),
    ).toBe("DELETE FROM users WHERE id IN ($id__0)");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new BulkDeleteStatement("users", ["id", "org"], [[{ name: "id", value: 1 }, { name: "org", value: 2 }]]),
      ),
    ).toBe("DELETE FROM users WHERE (id, org) IN (($id__0, $org__0))");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new AlterTableStatement("users", {
          kind: "alterColumn",
          column: { name: "age", type: "INTEGER", notNull: new NotNull("nn_age") },
        }),
      ),
    ).toBe("ALTER TABLE users ALTER COLUMN age TYPE INTEGER CONSTRAINT nn_age NOT NULL");
    expect(
      DefaultStatementTextRenderer.instance.render(
        new AlterTableStatement("users", { kind: "alterColumn", column: { name: "age", type: "INTEGER" } }),
      ),
    ).toBe("ALTER TABLE users ALTER COLUMN age TYPE INTEGER");
    const text = new StatementTextRenderer(
      (renderer) => new StatementRenderer(renderer),
      (expressions) => new ConditionRenderer(expressions),
      (renderer) => new ExpressionRenderer(renderer),
    );
    class CustomWhere extends Where {}
    expect(text.render(new CustomWhere(new Operator.Equals("id", 1)))).toBe("id = $id");
    class AlienWhere {
      constructor(readonly condition = new Operator.Equals("id", 1)) {}
      accept = (visitor: { visit: (where: AlienWhere) => string }) => visitor.visit(this);
    }
    let rendered = "";
    try {
      rendered = text.render(new AlienWhere() as never);
    } catch {
      rendered = "";
    }
    expect(rendered).not.toBe("id = $id");
    class Equals {
      constructor(
        readonly left = { name: "id" },
        readonly right = { value: 1 },
      ) {}
    }
    expect(() => new ExpressionRenderer(text).render(new Equals() as never)).toThrow(/empty expression/);
    expect(
      new ExpressionRenderer(text).render(new Operator.In(new Identifier("id"), new Identifier("ids"))),
    ).toBe("id IN ids");
    expect(() =>
      new ExpressionRenderer(text).render(new Operator.NotIn(new Identifier("id"), new Identifier("ids"))),
    ).toThrow(/empty expression/);
    const conditions = new ConditionRenderer(new ExpressionRenderer(text));
    expect(conditions.render(new WhereText("id = 1"))).toBe("id = 1");
    expect(() => conditions.visit(new AlienWhere() as never)).toThrow(/empty expression/);
    expect(() =>
      DefaultStatementTextRenderer.instance.render(new AlterTableStatement("users", { kind: "empty" } as never)),
    ).toThrow(/empty alter/);
    class QueryLike {
      where = true;
      accept = (visitor: { visit: (statement: object) => string }) => visitor.visit(this);
    }
    expect(() => DefaultStatementTextRenderer.instance.render(new QueryLike() as never)).toThrow();
  });

  it("renders a Where from a second module copy", async () => {
    const url = new URL("./Where.ts", import.meta.url);
    url.searchParams.set("copy", "2");
    const { Where: ForeignWhere } = await import(url.href);
    const foreign = new ForeignWhere(new Operator.Equals("id", 1));
    expect(foreign instanceof Where).toBe(false);
    expect(DefaultStatementTextRenderer.instance.render(foreign)).toBe("id = $id");
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
