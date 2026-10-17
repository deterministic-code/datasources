import {
  AlterTableStatement,
  ColumnDefault,
  CreateReadStoredProcedureStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DefaultStatementTextRenderer,
  ForeignKey,
  NotNull,
  PrimaryKey,
  ReferentialAction,
  SelectInsertedIdStatement,
  SelectStatement,
  StatementTextRenderer,
  UniqueConstraint,
} from "@datasources/core";
import { describe, expect, it } from "vitest";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";
import { SqliteStatementRenderer } from "./SqliteStatementRenderer.ts";

describe("SqliteStatementRenderer", () => {
  const renderer = new StatementTextRenderer(t => new SqliteStatementRenderer(t));

  it("renders inserted id and create table", () => {
    expect(renderer.render(new SelectInsertedIdStatement("users", "id"))).toBe(
      "SELECT last_insert_rowid() AS id",
    );

    const create = new CreateTableStatement(
      "orders",
      [
        { name: "id", type: "INTEGER", default: new ColumnDefault("0", "df_id"), notNull: new NotNull("nn_id") },
        { name: "user_id", type: "INTEGER" },
      ],
      [
        new PrimaryKey("pk_orders", ["id"]),
        new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade),
      ],
    );
    expect(renderer.render(create)).toBe(new SqliteCreateTableStatementRenderer().render(create));
    expect(renderer.render(new SelectStatement({ table: "users", fields: ["id"] }))).toBe(
      "SELECT id FROM users",
    );
  });

  it("renders canonical types", () => {
    const create = new CreateTableStatement("users", [
      { name: "id", type: "int", default: new ColumnDefault("0"), notNull: new NotNull(), identity: true },
    ]);
    expect(renderer.render(create)).toBe(
      "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT DEFAULT 0 NOT NULL)",
    );
    expect(DefaultStatementTextRenderer.instance.render(create)).toBe(
      "CREATE TABLE users (id int DEFAULT 0 NOT NULL)",
    );

    const keyed = new CreateTableStatement(
      "users",
      [{ name: "id", type: "int", identity: true }],
      [new PrimaryKey("pk_users", ["id"])],
    );
    expect(renderer.render(keyed)).toBe("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT)");
  });

  it("renders alter add column and throws on alter column", () => {
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "addColumn", column: { name: "email", type: "TEXT", notNull: new NotNull() } }),
      ),
    ).toBe("ALTER TABLE users ADD COLUMN email TEXT NOT NULL");
    expect(
      renderer.render(new AlterTableStatement("users", { kind: "dropColumn", name: "email" })),
    ).toBe("ALTER TABLE users DROP COLUMN email");
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "renameColumn", from: "old", to: "new" }),
      ),
    ).toBe("ALTER TABLE users RENAME COLUMN old TO new");
    expect(
      renderer.render(new AlterTableStatement("users", { kind: "renameTable", to: "people" })),
    ).toBe("ALTER TABLE users RENAME TO people");
    expect(() =>
      renderer.render(new AlterTableStatement("users", { kind: "alterColumn", column: { name: "age", type: "INTEGER" } })),
    ).toThrow(/SQLite cannot ALTER COLUMN/);
    expect(() =>
      renderer.render(
        new AlterTableStatement("users", { kind: "addConstraint", constraint: new UniqueConstraint("uq_email", ["email"]) }),
      ),
    ).toThrow(/SQLite cannot ADD CONSTRAINT/);
    expect(() =>
      renderer.render(new AlterTableStatement("users", { kind: "dropConstraint", name: "uq_email" })),
    ).toThrow(/SQLite cannot DROP CONSTRAINT/);
    expect(() =>
      renderer.render(new AlterTableStatement("users", { kind: "empty" } as never)),
    ).toThrow(/empty alter/);
    expect(
      renderer.render(
        new CreateReadStoredProcedureStatement("ReadUsers", "users", ["id"], [], "id", "INT", [], []),
      ),
    ).toBe("");
    expect(renderer.render(new CreateTriggerStatement("users", "tr_users", "updated", "id", "CURRENT_TIMESTAMP"))).toBe(
      `CREATE TRIGGER tr_users
AFTER UPDATE ON users
FOR EACH ROW
WHEN NEW.updated IS OLD.updated
BEGIN
  UPDATE users SET updated = CURRENT_TIMESTAMP WHERE id = OLD.id;
END`,
    );
  });
});
