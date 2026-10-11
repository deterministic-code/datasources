import {
  BulkUpdateStatement,
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
import { MysqlCreateTableStatementRenderer } from "./MysqlCreateTableStatementRenderer.ts";
import { MysqlStatementRenderer } from "./MysqlStatementRenderer.ts";

describe("MysqlStatementRenderer", () => {
  const renderer = new StatementTextRenderer(t => new MysqlStatementRenderer(t));

  it("renders inserted id and create table", () => {
    expect(renderer.render(new SelectInsertedIdStatement("users", "id"))).toBe(
      "SELECT LAST_INSERT_ID() AS id",
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
    expect(renderer.render(create)).toBe(new MysqlCreateTableStatementRenderer().render(create));
    expect(renderer.render(new SelectStatement({ table: "users", fields: ["id"] }))).toBe(
      "SELECT id FROM users",
    );
  });

  it("renders type default and identity", () => {
    const create = new CreateTableStatement("users", [
      { name: "id", type: "int", default: new ColumnDefault("0"), notNull: new NotNull(), identity: true },
    ]);
    expect(renderer.render(create)).toBe(
      "CREATE TABLE users (id INT AUTO_INCREMENT DEFAULT 0 NOT NULL)",
    );
    expect(DefaultStatementTextRenderer.instance.render(create)).toBe(
      "CREATE TABLE users (id int DEFAULT 0 NOT NULL)",
    );
  });

  it("renders bulk update as join", () => {
    const update = new BulkUpdateStatement("parents", ["label"], ["id"], [
      { fields: [{ name: "label", value: "a" }], ids: [{ name: "id", value: 1 }] },
      { fields: [{ name: "label", value: "b" }], ids: [{ name: "id", value: 2 }] },
    ]);
    expect(renderer.render(update)).toBe(
      "UPDATE parents JOIN (SELECT $label__0 AS label, $id__0 AS id UNION ALL SELECT $label__1 AS label, $id__1 AS id) AS data ON parents.id = data.id SET parents.label = data.label",
    );
  });
});
