import {
  ColumnDefault,
  CreateTableStatement,
  ForeignKey,
  NotNull,
  PrimaryKey,
  ReferentialAction,
  SelectStatement,
} from "@datasources/core";
import { describe, expect, it } from "vitest";
import { SqliteStatementFactory } from "./SqliteStatementFactory.ts";

describe("SqliteStatementFactory", () => {
  it("transforms a create table and passes other statements through", () => {
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
    const factory = new SqliteStatementFactory();
    const transformed = factory.transform(create);
    expect(transformed).toBeInstanceOf(CreateTableStatement);
    expect((transformed as CreateTableStatement).table).toBeUndefined();
    expect(transformed.getText()).toBe(create.getText());
    expect(factory.transform(transformed)).toBe(transformed);

    const raw = new CreateTableStatement("x");
    expect(factory.transform(raw)).toBe(raw);
    const select = new SelectStatement("SELECT id FROM users");
    expect(factory.transform(select)).toBe(select);
  });

  it("renders canonical types", () => {
    const create = new CreateTableStatement("users", [
      { name: "id", type: "int", default: new ColumnDefault("0"), notNull: new NotNull(), identity: true },
    ]);
    expect(new SqliteStatementFactory().transform(create).getText()).toBe(
      "CREATE TABLE users (id INTEGER DEFAULT 0 NOT NULL)",
    );
    expect(create.getText()).toBe("CREATE TABLE users (id int DEFAULT 0 NOT NULL)");
  });
});
