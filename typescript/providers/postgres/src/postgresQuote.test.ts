import { describe, expect, it } from "vitest";
import {
  DeleteStatement,
  InsertStatement,
  Operator,
  QueryStatement,
  ExecuteStoredProcedureStatement,
  UpdateStatement,
  WhereClause,
} from "@datasources/core";
import { PostgresQuote } from "./postgresQuote.ts";

describe("PostgresQuote", () => {
  it("quotes and qualifies identifiers", () => {
    expect(PostgresQuote.quote("user")).toBe('"user"');
    expect(PostgresQuote.quote('a"b')).toBe('"a""b"');
    expect(PostgresQuote.qualified("users.id")).toBe('"users"."id"');
  });

  it("quotes tables in DML", () => {
    const where = new WhereClause(new Operator.Equals("id", 1));
    const fields = [
      { name: "id", value: 1 },
      { name: "username", value: "alice" },
    ];
    expect(
      new QueryStatement(
        PostgresQuote.quote("user"),
        [PostgresQuote.quote("id"), PostgresQuote.quote("username")],
        where,
      ).getText(),
    ).toBe('SELECT "id", "username" FROM "user" WHERE id = $id');
    expect(new InsertStatement(PostgresQuote.quote("user"), fields).getText()).toBe(
      'INSERT INTO "user" (id, username) VALUES ($id, $username)',
    );
    expect(new UpdateStatement(PostgresQuote.quote("user"), fields, where).getText()).toBe(
      'UPDATE "user" SET id = $id, username = $username WHERE id = $id',
    );
    expect(new DeleteStatement(PostgresQuote.quote("user"), where).getText()).toBe(
      'DELETE FROM "user" WHERE id = $id',
    );
    expect(new ExecuteStoredProcedureStatement(PostgresQuote.quote("save"), fields).getText()).toBe(
      'CALL "save"($id, $username)',
    );
  });
});
