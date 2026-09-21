import { describe, expect, it } from "vitest";
import {
  DeleteStatement,
  InsertStatement,
  Operator,
  QueryStatement,
  StoredProcedureStatement,
  UpdateStatement,
  WhereClause,
} from "@datasources/core";
import { MysqlQuote } from "./mysqlQuote.ts";

describe("MysqlQuote", () => {
  it("quotes and qualifies identifiers", () => {
    expect(MysqlQuote.quote("user")).toBe("`user`");
    expect(MysqlQuote.quote("a`b")).toBe("`a``b`");
    expect(MysqlQuote.qualified("users.id")).toBe("`users`.`id`");
  });

  it("quotes tables in DML", () => {
    const where = new WhereClause(new Operator.Equals("id", 1));
    const fields = [
      { name: "id", value: 1 },
      { name: "username", value: "alice" },
    ];
    expect(
      new QueryStatement(MysqlQuote.quote("user"), [MysqlQuote.quote("id"), MysqlQuote.quote("username")], where).getText(),
    ).toBe("SELECT `id`, `username` FROM `user` WHERE id = $id");
    expect(new InsertStatement(MysqlQuote.quote("user"), fields).getText()).toBe(
      "INSERT INTO `user` (id, username) VALUES ($id, $username)",
    );
    expect(new UpdateStatement(MysqlQuote.quote("user"), fields, where).getText()).toBe(
      "UPDATE `user` SET id = $id, username = $username WHERE id = $id",
    );
    expect(new DeleteStatement(MysqlQuote.quote("user"), where).getText()).toBe(
      "DELETE FROM `user` WHERE id = $id",
    );
    expect(new StoredProcedureStatement(MysqlQuote.quote("save"), fields).getText()).toBe(
      "CALL `save`($id, $username)",
    );
  });
});
