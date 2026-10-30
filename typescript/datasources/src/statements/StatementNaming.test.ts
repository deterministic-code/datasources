import { describe, expect, it } from "vitest";
import { CreateIndexStatement } from "./CreateIndexStatement.ts";
import { CreateTableStatement } from "./CreateTableStatement.ts";
import { CreateTriggerStatement } from "./CreateTriggerStatement.ts";
import { ForeignKey } from "./ForeignKey.ts";
import { ForeignKeyStatement } from "./ForeignKeyStatement.ts";
import { KeywordCasing } from "./KeywordCasing.ts";
import { PrimaryKey } from "./PrimaryKey.ts";
import { procedureNameOf, sqlProcedureStatementsOf } from "./SqlProcedureStatements.ts";
import { SelectStatement } from "./SelectStatement.ts";
import { StatementNaming } from "./StatementNaming.ts";
import { StatementRenderer } from "./StatementRenderer.ts";
import { StatementTextRenderer } from "./StatementTextRenderer.ts";
import { UniqueConstraint } from "./UniqueConstraint.ts";
import type { ProcedureSource } from "./ProcedureSource.ts";

const source = (routineName: (stem: string) => string = (stem) => stem): ProcedureSource => ({
  plural: "users",
  table: "users",
  columns: ["id", "email"],
  createParameters: [{ name: "email", type: "string" }],
  writable: ["email"],
  createColumns: [{ name: "email", type: "string" }],
  updateColumns: [{ name: "id", type: "integer" }, { name: "email", type: "string" }],
  typedColumns: [{ name: "id", type: "integer" }, { name: "email", type: "string" }],
  byGroups: [{
    logical: ["id"],
    columns: ["id"],
    parameters: [{ name: "id", type: "integer" }],
    updateParameters: [{ name: "id", type: "integer" }, { name: "email", type: "string" }],
  }],
  key: "id",
  keyType: "integer",
  generatedKey: false,
  routineName,
});

describe("StatementNaming", () => {
  it("keeps verbatim stems and pluralizes snake tokens", () => {
    const naming = StatementNaming.sql;
    expect(naming.isVerbatim("OldRvwsTbl")).toBe(true);
    expect(naming.physicalStem("OldRvwsTbl")).toBe("OldRvwsTbl");
    expect(naming.physicalStem("notification_type")).toBe("notification_types");
    expect(naming.physicalStem("user")).toBe("users");
    expect(new StatementNaming({ pluralizeTables: false }).physicalStem("user")).toBe("user");
  });

  it("converts identifier casings without splitting digits", () => {
    const naming = StatementNaming.sql;
    expect(naming.convert("", "snakeCase")).toBe("");
    expect(naming.convert("line1", "snakeCase")).toBe("line1");
    expect(naming.convert("channel_name", "camelCase")).toBe("channelName");
    expect(naming.convert("channel_name", "pascalCase")).toBe("ChannelName");
    expect(naming.convert("channel_name", "kebabCase")).toBe("channel-name");
    expect(naming.convert("channel_name", "constantCase")).toBe("CHANNEL_NAME");
  });

  it("folds object and keyword letter-case", () => {
    expect(new StatementNaming({ objects: "upper" }).letter("users")).toBe("USERS");
    expect(new StatementNaming({ objects: "lower" }).letter("USERS")).toBe("users");
    expect(StatementNaming.sql.letter("users")).toBe("users");
    expect(StatementNaming.sql.keyword("create")).toBe("CREATE");
    expect(new StatementNaming({ keywordCasing: KeywordCasing.Lower }).keyword("CREATE")).toBe("create");
  });
});

describe("statement names", () => {
  it("names tables, columns, constraints, and triggers", () => {
    const naming = StatementNaming.sql;
    expect(CreateTableStatement.tableName("notification_type", naming)).toBe("notification_types");
    expect(CreateTableStatement.tableName("OldRvwsTbl", naming)).toBe("OldRvwsTbl");
    expect(CreateTableStatement.columnName("channel_name", naming)).toBe("channel_name");
    expect(CreateTableStatement.columnName("email_address", naming)).toBe("email_address");
    expect(CreateTableStatement.columnName("EmailAddress", naming)).toBe("EmailAddress");
    expect(PrimaryKey.nameOf("user", naming, "primary_key")).toBe("users_primary_key");
    expect(UniqueConstraint.nameOf("user", naming, "email", "unique_constraint")).toBe("users_email_unique_constraint");
    expect(ForeignKey.nameOf("user", naming, "id", "foreign_key")).toBe("users_id_foreign_key");
    expect(ForeignKeyStatement.nameOf("OldRvwsTbl", naming, "id", "foreign_key")).toBe("OldRvwsTbl_id_foreign_key");
    expect(CreateIndexStatement.nameOf("user", naming, "owner", "index")).toBe("users_owner_index");
    expect(CreateTriggerStatement.nameOf("user", naming)).toBe("trg_users_updated_at");
    expect(CreateTriggerStatement.nameOf("OldRvwsTbl", naming)).toBe("trg_OldRvwsTbl_updated_at");
  });
});

describe("StatementTextRenderer naming", () => {
  it("copies factories onto a named renderer and applies keywords when lower", () => {
    const apply = (sql: string, keyword: (word: string) => string) =>
      sql.replace("CREATE", keyword("CREATE"));
    const renderer = new StatementTextRenderer(undefined, undefined, undefined, undefined, apply);
    const lower = renderer.withNaming(new StatementNaming({ keywordCasing: KeywordCasing.Lower }));
    expect(lower.applyKeywords("CREATE TABLE t")).toBe("create TABLE t");
    expect(renderer.applyKeywords("CREATE TABLE t")).toBe("CREATE TABLE t");
    expect(new StatementTextRenderer(
      undefined,
      undefined,
      undefined,
      new StatementNaming({ keywordCasing: KeywordCasing.Lower }),
    ).applyKeywords("CREATE TABLE t")).toBe("CREATE TABLE t");
  });

  it("returns dialect procedures from the statement renderer", () => {
    class Procedures extends StatementRenderer {
      override procedures = (input: ProcedureSource) => sqlProcedureStatementsOf(input);
    }
    const renderer = new StatementTextRenderer((text) => new Procedures(text));
    const statements = renderer.procedures(source());
    expect(statements.map(procedureNameOf)).toEqual([
      "create_users",
      "read_users",
      "read_users_by_id",
      "update_users_by_id",
      "delete_users_by_id",
      "bulk_create_users",
      "bulk_update_users",
      "bulk_delete_users",
    ]);
    expect(new StatementTextRenderer().procedures(source())).toEqual([]);
    expect(new StatementTextRenderer(() => ({ visit: () => "" }) as unknown as StatementRenderer).procedures(source())).toEqual([]);
    expect(() => procedureNameOf(new SelectStatement({ table: "users", fields: ["id"] }))).toThrow("procedure statement has no name");
  });
});
