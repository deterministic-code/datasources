import {
  AlterTableStatement,
  ColumnDefault,
  CreateTableStatement,
  DefaultStatementTextRenderer,
  ForeignKey,
  SelectInsertedIdStatement,
  NotNull,
  PrimaryKey,
  ReferentialAction,
  SelectStatement,
  StatementTextRenderer,
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
    expect(() =>
      renderer.render(new AlterTableStatement("users", { kind: "alterColumn", column: { name: "age", type: "INTEGER" } })),
    ).toThrow(/SQLite cannot ALTER COLUMN/);
  });
});
