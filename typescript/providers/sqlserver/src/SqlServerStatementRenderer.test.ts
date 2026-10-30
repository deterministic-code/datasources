import {
  AlterTableStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateReadStoredProcedureStatement,
  CreateUpdateStoredProcedureStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  DropIndexStatement,
  DropRoutineStatement,
  IdentityInsertStatement,
  InsertStatement,
  NotNull,
  StatementTextRenderer,
} from "@deterministic-code/datasources";
import { describe, expect, it } from "vitest";
import { SqlServerStatementRenderer } from "./SqlServerStatementRenderer.ts";

describe("SqlServerStatementRenderer", () => {
  const renderer = new StatementTextRenderer(t => new SqlServerStatementRenderer(t));

  it("renders create, alter, trigger, identity insert, and drop statements", () => {
    expect(
      renderer.render(new CreateTableStatement("users", [{ name: "id", type: "INTEGER", identity: true }])),
    ).toBe("CREATE TABLE users (id INTEGER IDENTITY(1,1))");
    expect(
      renderer.render(new CreateTableStatement("users", [{ name: "id", type: "INTEGER" }])),
    ).toBe("CREATE TABLE users (id INTEGER)");
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "renameColumn", from: "old", to: "new" }),
      ),
    ).toBe("EXEC sp_rename 'users.old', 'new', 'COLUMN'");
    expect(
      renderer.render(new AlterTableStatement("users", { kind: "renameTable", to: "people" })),
    ).toBe("EXEC sp_rename 'users', 'people'");
    expect(
      renderer.render(
        new AlterTableStatement("users", {
          kind: "alterColumn",
          column: { name: "age", type: "INTEGER", notNull: new NotNull() },
        }),
      ),
    ).toBe("ALTER TABLE users ALTER COLUMN age INTEGER NOT NULL");
    expect(
      renderer.render(
        new AlterTableStatement("users", { kind: "addColumn", column: { name: "email", type: "TEXT" } }),
      ),
    ).toBe("ALTER TABLE [users] ADD COLUMN [email] TEXT");
    expect(renderer.render(new DropIndexStatement("ix_users_email", "users"))).toBe(
      "DROP INDEX [ix_users_email] ON [users]",
    );
    expect(renderer.render(new CreateTriggerStatement("users", "tr_users", "updated", "id"))).toBe(
      `CREATE TRIGGER [tr_users] ON [users]
AFTER UPDATE AS
BEGIN
  SET NOCOUNT ON;
  UPDATE [users] SET [updated] = GETUTCDATE()
  FROM [users] t INNER JOIN inserted i ON t.[id] = i.[id];
END`,
    );
    const insert = new InsertStatement("users", [{ name: "id", value: 1 }, { name: "name", value: "ada" }]);
    expect(renderer.render(new IdentityInsertStatement("users", "id", [insert], true))).toBe(
      `SET IDENTITY_INSERT users ON;
INSERT INTO users (id, name) VALUES (1, ada);
SET IDENTITY_INSERT users OFF`,
    );
    expect(renderer.render(new IdentityInsertStatement("users", "id", [insert]))).toBe(
      "INSERT INTO users (id, name) VALUES (1, ada);",
    );
    expect(renderer.render(new IdentityInsertStatement("users", "id", [], true))).toBe("");
    expect(renderer.render(new DropRoutineStatement("sp_users"))).toBe("DROP PROCEDURE IF EXISTS sp_users");
  });

  it("renders create/read/update stored procedure statements", () => {
    expect(
      renderer.render(
        new CreateCreateStoredProcedureStatement("CreateUsers", "users", ["id", "name"], [{ name: "name", type: "TEXT" }], "id", "INT", ["name"]),
      ),
    ).toContain("CREATE OR ALTER PROCEDURE CreateUsers");
    expect(
      renderer.render(
        new CreateReadStoredProcedureStatement("ReadUsers", "users", ["id", "name"], [], "id", "INT", [], []),
      ),
    ).toContain("ORDER BY id");
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
    ).toContain("WHERE id = @id");
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
    ).toContain("id = @id AND version = @version");
    expect(
      renderer.render(
        new CreateBulkDeleteStoredProcedureStatement(
          "BulkDeleteUsers",
          "users",
          [{ name: "id", type: "INT" }],
          { name: "payload", type: "NVARCHAR(MAX)" },
          "id",
          "INT",
        ),
      ),
    ).toContain("OPENJSON");
  });
});
