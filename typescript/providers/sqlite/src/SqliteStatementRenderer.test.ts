import {
  AlterTableStatement,
  ColumnDefault,
  CreateIndexStatement,
  CreateReadStoredProcedureStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DefaultStatementTextRenderer,
  DropIndexStatement,
  DropTableStatement,
  DropTriggerStatement,
  ForeignKey,
  ForeignKeyStatement,
  NotNull,
  PrimaryKey,
  ReferentialAction,
  SelectInsertedIdStatement,
  SelectStatement,
  SqlSeedStatement,
  StatementTextRenderer,
  UniqueConstraint,
} from "@deterministic-code/datasources";
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
      'CREATE TABLE "users" ("id" INTEGER PRIMARY KEY AUTOINCREMENT DEFAULT 0 NOT NULL)',
    );
    expect(DefaultStatementTextRenderer.instance.render(create)).toBe(
      "CREATE TABLE users (id int DEFAULT 0 NOT NULL)",
    );

    const keyed = new CreateTableStatement(
      "users",
      [{ name: "id", type: "int", identity: true }],
      [new PrimaryKey("pk_users", ["id"])],
    );
    expect(renderer.render(keyed)).toBe('CREATE TABLE "users" ("id" INTEGER PRIMARY KEY AUTOINCREMENT)');
  });

  it("renders alter add column and throws on alter column", () => {
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "addColumn", column: { name: "email", type: "TEXT", notNull: new NotNull() } }),
      ),
    ).toBe('ALTER TABLE "users" ADD COLUMN "email" TEXT NOT NULL');
    expect(
      renderer.render(new AlterTableStatement("users", { kind: "dropColumn", name: "email" })),
    ).toBe('ALTER TABLE "users" DROP COLUMN "email"');
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "renameColumn", from: "old", to: "new" }),
      ),
    ).toBe('ALTER TABLE "users" RENAME COLUMN "old" TO "new"');
    expect(
      renderer.render(new AlterTableStatement("users", { kind: "renameTable", to: "people" })),
    ).toBe('ALTER TABLE "users" RENAME TO "people"');
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
    expect(renderer.render(new CreateTriggerStatement("users", "tr_users", "updated", "id"))).toBe(
      `CREATE TRIGGER "tr_users"
AFTER UPDATE ON "users"
FOR EACH ROW
WHEN NEW."updated" IS OLD."updated"
BEGIN
  UPDATE "users" SET "updated" = (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) WHERE "id" = OLD."id";
END`,
    );
  });

  it("renders seed insert and delete literals", () => {
    expect(
      renderer.render(
        new SqlSeedStatement("users", [
          { column: "id", field: { type: "integer" }, value: 1 },
          { column: "name", field: { type: "string" }, value: "O'Brien" },
          { column: "flag", field: { type: "boolean" }, value: true },
          { column: "off", field: { type: "boolean" }, value: false },
          { column: "blob", field: { type: "binary" }, value: "ab" },
          { column: "missing", field: { type: "string" }, value: null },
          {
            column: "created",
            field: { type: "datetime", defaultValue: "UtcNow" },
            value: null,
          },
          { column: "local", field: { type: "datetime", defaultValue: "Now" }, value: null },
          { column: "uid", field: { type: "uuid", defaultValue: "NewId" }, value: null },
          { column: "ok", field: { type: "boolean", defaultValue: "true" }, value: null },
          { column: "no", field: { type: "boolean", defaultValue: "false" }, value: null },
          { column: "label", field: { type: "string", defaultValue: "x" }, value: null },
        ]),
      ),
    ).toContain("'x'");
    expect(
      renderer.render(
        new SqlSeedStatement(
          "users",
          [{ column: "id", field: { type: "integer" }, value: 1 }],
          "delete",
        ),
      ),
    ).toBe('DELETE FROM "users" WHERE "id" = 1');
    expect(
      renderer.render(
        new SqlSeedStatement("flags", [
          { column: "a", field: { type: "boolean" }, value: "true" },
          { column: "b", field: { type: "boolean" }, value: "True" },
          { column: "c", field: { type: "boolean" }, value: 1 },
        ]),
      ),
    ).toBe('INSERT INTO "flags" ("a", "b", "c") VALUES (1, 1, 1)');
    expect(renderer.render(new DropTableStatement("users"))).toBe('DROP TABLE IF EXISTS "users"');
    expect(renderer.render(new DropTriggerStatement("tr_users", "users"))).toBe(
      'DROP TRIGGER IF EXISTS "tr_users"',
    );
    expect(renderer.render(new CreateIndexStatement("ix_name", "users", ["name"], true))).toBe(
      'CREATE UNIQUE INDEX "ix_name" ON "users" ("name")',
    );
    expect(renderer.render(new DropIndexStatement("ix_name", "users"))).toBe(
      'DROP INDEX IF EXISTS "ix_name"',
    );
    expect(() =>
      renderer.render(new ForeignKeyStatement("users", "fk", ["id"], "other", ["id"], "add")),
    ).toThrow(/SQLite cannot ADD CONSTRAINT/);
    expect(() =>
      renderer.render(new ForeignKeyStatement("users", "fk", ["id"], "other", ["id"], "drop")),
    ).toThrow(/SQLite cannot DROP CONSTRAINT/);
    expect(
      renderer.render(
        new AlterTableStatement("users", {
          kind: "addColumn",
          column: { name: "age", type: "INTEGER", default: new ColumnDefault("0") },
        }),
      ),
    ).toBe('ALTER TABLE "users" ADD COLUMN "age" INTEGER DEFAULT 0');
    expect(
      renderer.render(
        new AlterTableStatement("users", {
          kind: "addColumn",
          column: { name: "age", type: "INTEGER", default: new ColumnDefault("0", "df_age") },
        }),
      ),
    ).toBe('ALTER TABLE "users" ADD COLUMN "age" INTEGER CONSTRAINT "df_age" DEFAULT 0');
  });
});
