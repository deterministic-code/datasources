import {
  AlterTableStatement,
  BulkUpdateStatement,
  ColumnDefault,
  CreateBulkCreateStoredProcedureStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateBulkUpdateStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateDeleteStoredProcedureStatement,
  CreateReadStoredProcedureStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  CreateUpdateStoredProcedureStatement,
  DefaultStatementTextRenderer,
  DropIndexStatement,
  DropRoutineStatement,
  ForeignKey,
  NotNull,
  PrimaryKey,
  ReferentialAction,
  SelectInsertedIdStatement,
  SelectStatement,
  StatementTextRenderer,
} from "@deterministic-code/datasources";
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

  it("renders alter, trigger, and drop statements", () => {
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "alterColumn", column: { name: "id", type: "int", identity: true } }),
      ),
    ).toBe("ALTER TABLE users MODIFY COLUMN id INT AUTO_INCREMENT");
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "addColumn", column: { name: "email", type: "TEXT" } }),
      ),
    ).toBe("ALTER TABLE users ADD COLUMN email TEXT");
    expect(renderer.render(new DropIndexStatement("ix_users_email", "users"))).toBe(
      "DROP INDEX ix_users_email ON users",
    );
    expect(renderer.render(new CreateTriggerStatement("users", "tr_users", "updated", "id", "UTC_TIMESTAMP()"))).toBe(
      "CREATE TRIGGER tr_users BEFORE UPDATE ON users\nFOR EACH ROW SET NEW.updated = UTC_TIMESTAMP()",
    );
    expect(renderer.render(new DropRoutineStatement("sp_users"))).toBe("DROP PROCEDURE IF EXISTS sp_users");
  });

  it("renders create/read/update stored procedure statements", () => {
    expect(
      renderer.render(
        new CreateCreateStoredProcedureStatement("CreateUsers", "users", ["id", "name"], [{ name: "name", type: "TEXT" }], "id", "INT", ["name"]),
      ),
    ).toContain("CREATE PROCEDURE CreateUsers");
    expect(
      renderer.render(
        new CreateCreateStoredProcedureStatement(
          "CreateUsersGenerated",
          "users",
          ["id", "name"],
          [{ name: "name", type: "TEXT" }],
          "id",
          "VARCHAR(36)",
          ["name"],
          true,
        ),
      ),
    ).toContain("SET new_id = UUID()");
    expect(
      renderer.render(
        new CreateReadStoredProcedureStatement("ReadUsers", "users", ["id", "name"], [], "id", "INT", [], []),
      ),
    ).toContain("ORDER BY t.id");
    expect(
      renderer.render(
        new CreateReadStoredProcedureStatement(
          "ReadUsersById",
          "users",
          ["id", "name"],
          [{ name: "id", type: "INT" }],
          "id",
          "INT",
          [],
          ["id"],
        ),
      ),
    ).toContain("WHERE t.id = id");
    expect(
      renderer.render(
        new CreateUpdateStoredProcedureStatement(
          "UpdateUsersByIdVersion",
          "users",
          ["id", "name", "version"],
          [{ name: "id", type: "INT" }, { name: "version", type: "INT" }, { name: "name", type: "TEXT" }],
          "id",
          "INT",
          ["name"],
          ["id", "version"],
        ),
      ),
    ).toContain("WHERE t.id = id AND t.version = version");
    expect(
      renderer.render(
        new CreateUpdateStoredProcedureStatement(
          "UpdateUsersAudit",
          "users",
          ["id", "name", "updated"],
          [{ name: "id", type: "INT" }, { name: "name", type: "TEXT" }, { name: "new_updated", type: "DATETIME" }],
          "id",
          "INT",
          ["name"],
          ["id"],
          false,
          "updated",
        ),
      ),
    ).toContain("t.updated = new_updated");
    expect(
      renderer.render(
        new CreateUpdateStoredProcedureStatement(
          "UpdateUsersAuditMissing",
          "users",
          ["id", "name", "updated"],
          [{ name: "id", type: "INT" }, { name: "name", type: "TEXT" }],
          "id",
          "INT",
          ["name"],
          [],
          false,
          "updated",
        ),
      ),
    ).toContain("UPDATE users t");
    expect(
      renderer.render(
        new CreateDeleteStoredProcedureStatement(
          "DeleteUsersById",
          "users",
          ["id"],
          [{ name: "id", type: "BIGINT" }],
          "id",
          "INT",
          [],
          ["id"],
        ),
      ),
    ).toContain("IN id BIGINT");
    expect(
      renderer.render(
        new CreateDeleteStoredProcedureStatement("DeleteUsersByKey", "users", ["id"], [], "id", "INT", [], []),
      ),
    ).toContain("WHERE t.id = id");
    expect(
      renderer.render(
        new CreateReadStoredProcedureStatement("ReadUsersByName", "users", ["id", "name"], [], "id", "INT", [], ["name"]),
      ),
    ).toContain("IN name INT");
    expect(
      renderer.render(
        new CreateDeleteStoredProcedureStatement("DeleteUsersByName", "users", ["id"], [], "id", "INT", [], ["name"]),
      ),
    ).toContain("IN name INT");
    const payload = { name: "payload", type: "JSON" };
    expect(
      renderer.render(new CreateBulkCreateStoredProcedureStatement("BulkCreateUsers", "users", [payload], payload, "id", "INT")),
    ).toContain("JSON_TABLE");
    expect(
      renderer.render(new CreateBulkUpdateStoredProcedureStatement("BulkUpdateUsers", "users", [payload], payload, "id", "INT")),
    ).toContain("JSON_TABLE");
    expect(
      renderer.render(
        new CreateBulkDeleteStoredProcedureStatement(
          "BulkDeleteUsers",
          "users",
          [{ name: "id", type: "INT" }],
          { name: "payload", type: "JSON" },
          "id",
          "INT",
        ),
      ),
    ).toContain("JSON_TABLE");
    expect(
      renderer.render(
        new CreateBulkDeleteStoredProcedureStatement(
          "BulkDeleteUsersByKeyType",
          "users",
          [],
          { name: "payload", type: "JSON" },
          "id",
          "INT",
        ),
      ),
    ).toContain("id INT PATH '$.id'");
  });
});
