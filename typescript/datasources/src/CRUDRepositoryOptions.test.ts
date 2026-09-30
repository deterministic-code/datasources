import { describe, expect, it } from "vitest";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import { CRUDRepositoryStatements } from "./CRUDRepositoryStatements.ts";
import { CRUDRepositoryBuilder } from "./CRUDRepositoryBuilder.ts";
import { ReadonlyRepositoryBuilder } from "./ReadonlyRepositoryBuilder.ts";
import { MockConnection } from "./mocks/mock-connection.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";

describe("CRUDRepositoryOptions", () => {
  it("defaults are identity", () => {
    const read: CRUDRepositoryOptions = {
      ids: ["id"],
      entity: "users",
      readProperties: ["id"],
    };
    const built = new CRUDRepositoryBuilder().parts(new MockConnection(), read);
    expect(built.statements.select().getText()).toBe("SELECT id FROM users");
    expect(built.propertyMappings.map("x")).toBe("x");
    expect(built.propertyMappings.reverseMap("x")).toBe("x");
    const withId: CRUDRepositoryOptions = { ids: ["id"] };
    expect(new CRUDRepositoryBuilder().parts(new MockConnection(), withId).ids).toEqual(["id"]);
    const fromType: CRUDRepositoryOptions<{ id: number }> = {
      type: class {
        id = 0;
      },
      ids: ["id"],
    };
    expect(new CRUDRepositoryBuilder<{ id: number }>().parts(new MockConnection(), fromType).ids).toEqual(["id"]);
    expect(new ReadonlyRepositoryBuilder().parts(new MockConnection(), {}).ids).toEqual([]);
  });

  it("uses the mapped table and columns", () => {
    const statements = new CRUDRepositoryStatements("users", ["id", "name"]);
    expect(statements.select().getText()).toBe("SELECT id, name FROM users");
    expect(statements.insert([{ name: "id", value: 1 }]).getText()).toBe(
      "INSERT INTO users (id) VALUES ($id)",
    );
    expect(statements.update([{ name: "name", value: "a" }]).getText()).toBe(
      "UPDATE users SET name = $name",
    );
    expect(UpdateStatement.parameterList([{ name: "name", value: "a" }])).toEqual([
      { name: "name", value: "a" },
    ]);
    expect(statements.delete().getText()).toBe("DELETE FROM users");
    const inserted = statements.selectInsertedId("id");
    expect(inserted.table).toBe("users");
    expect(inserted.column).toBe("id");
    expect(() => inserted.getText()).toThrow("provider must implement SelectInsertIdStatement");
  });
});
