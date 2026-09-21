import { describe, expect, it } from "vitest";
import {
  DeleteStatement,
  InsertStatement,
  Join,
  Operator,
  QueryStatement,
  StoredProcedureStatement,
  UpdateStatement,
  WhereClause,
} from "@datasources/core";
import { SqliteQuote } from "./sqliteQuote.ts";

describe("SqliteQuote", () => {
  it("quotes and qualifies identifiers", () => {
    expect(SqliteQuote.quote("user")).toBe('"user"');
    expect(SqliteQuote.quote('a"b')).toBe('"a""b"');
    expect(SqliteQuote.qualified("users.id")).toBe('"users"."id"');
  });

  it("quotes tables in DML", () => {
    const where = new WhereClause(new Operator.Equals("id", 1));
    const fields = [
      { name: "id", value: 1 },
      { name: "username", value: "alice" },
    ];
    expect(
      new QueryStatement(SqliteQuote.quote("user"), [SqliteQuote.quote("id"), SqliteQuote.quote("username")], where).getText(),
    ).toBe('SELECT "id", "username" FROM "user" WHERE id = $id');
    expect(new InsertStatement(SqliteQuote.quote("user"), fields).getText()).toBe(
      'INSERT INTO "user" (id, username) VALUES ($id, $username)',
    );
    expect(new UpdateStatement(SqliteQuote.quote("user"), fields, where).getText()).toBe(
      'UPDATE "user" SET id = $id, username = $username WHERE id = $id',
    );
    expect(new DeleteStatement(SqliteQuote.quote("user"), where).getText()).toBe(
      'DELETE FROM "user" WHERE id = $id',
    );
    expect(new StoredProcedureStatement(SqliteQuote.quote("save"), fields).getText()).toBe(
      'CALL "save"($id, $username)',
    );
    expect(
      new QueryStatement(
        SqliteQuote.quote("users"),
        [SqliteQuote.qualified("users.id"), SqliteQuote.qualified("orders.total")],
        new WhereClause(new Operator.Equals("users.id", 1)),
        [
          Join.inner(SqliteQuote.qualified("orders"), SqliteQuote.qualified("users.id"), SqliteQuote.qualified("orders.user_id")),
          Join.left(SqliteQuote.qualified("payments"), SqliteQuote.qualified("orders.id"), SqliteQuote.qualified("payments.order_id")),
        ],
      ).getText(),
    ).toBe(
      'SELECT "users"."id", "orders"."total" FROM "users" INNER JOIN "orders" ON "users"."id" = "orders"."user_id" LEFT JOIN "payments" ON "orders"."id" = "payments"."order_id" WHERE users.id = $users.id',
    );
  });
});
