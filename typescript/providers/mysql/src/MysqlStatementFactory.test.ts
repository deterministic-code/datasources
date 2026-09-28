import {
  CheckConstraint,
  ColumnDefault,
  CreateTableStatement,
  ForeignKey,
  InsertedIdStatement,
  NotNull,
  PrimaryKey,
  ReferentialAction,
  SelectStatement,
  UniqueConstraint,
  type ITableConstraint,
} from "@datasources/core";
import { describe, expect, it } from "vitest";
import { MysqlStatementFactory } from "./MysqlStatementFactory.ts";

describe("MysqlStatementFactory", () => {
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
    const factory = new MysqlStatementFactory();
    const transformed = factory.transform(create);
    expect(transformed).toBeInstanceOf(CreateTableStatement);
    expect((transformed as CreateTableStatement).table).toBeUndefined();
    expect(transformed.getText()).toBe(create.getText());
    expect(factory.transform(transformed)).toBe(transformed);

    const raw = new CreateTableStatement("x");
    expect(factory.transform(raw)).toBe(raw);
    const select = new SelectStatement("SELECT id FROM users");
    expect(factory.transform(select)).toBe(select);
    expect(factory.transform(new InsertedIdStatement("users", "id")).getText()).toBe(
      "SELECT LAST_INSERT_ID() AS id",
    );

    const plain = new CreateTableStatement("users", [
      { name: "id", type: "INTEGER", default: new ColumnDefault("0"), notNull: new NotNull() },
    ]);
    expect(factory.transform(plain).getText()).toBe(plain.getText());
    const extra = new CreateTableStatement("t", [{ name: "id", type: "INTEGER" }], [
      new UniqueConstraint("uq", ["id"]),
      new CheckConstraint("chk", "id > 0"),
      new ForeignKey("fk_null", ["id"], "u", ["id"], ReferentialAction.SetNull, ReferentialAction.SetDefault),
      new ForeignKey("fk_restrict", ["id"], "u", ["id"], ReferentialAction.Restrict, ReferentialAction.NoAction),
    ]);
    expect(factory.transform(extra).getText()).toBe(extra.getText());
    expect(() =>
      factory.transform(
        new CreateTableStatement("t", [{ name: "id", type: "INTEGER" }], [
          { kind: "other", name: "x" } as ITableConstraint,
        ]),
      ),
    ).toThrow("empty constraint");
    const identity = new CreateTableStatement("users", [
      { name: "id", type: "int", default: new ColumnDefault("0"), notNull: new NotNull(), identity: true },
    ]);
    expect(factory.transform(identity).getText()).toBe(
      "CREATE TABLE users (id INT AUTO_INCREMENT DEFAULT 0 NOT NULL)",
    );
    expect(identity.getText()).toBe("CREATE TABLE users (id int DEFAULT 0 NOT NULL)");
  });
});
