import { describe, expect, it } from "vitest";
import { DefaultStatementTextRenderer, Operator, UpdateStatement, Where } from "./statements/index.ts";
import { CRUDRepositoryStatements } from "./CRUDRepositoryStatements.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
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
    expect(DefaultStatementTextRenderer.instance.render(built.statements.select())).toBe("SELECT id FROM users");
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
    expect(new CRUDRepositoryBuilder().parts(new MockConnection(), { entity: "users" }).ids).toEqual([]);
    expect(
      new CRUDRepositoryBuilder().parts(new MockConnection(), {
        write: new DefaultWriteEnrichmentStrategy(),
      }).ids,
    ).toEqual(["id"]);
    class Named {
      id = 0;
    }
    expect(
      new CRUDRepositoryBuilder<Named>().parts(new MockConnection(), {
        type: Named,
        write: new DefaultWriteEnrichmentStrategy<Named>(),
      }).ids,
    ).toEqual(["id"]);
    expect(
      new CRUDRepositoryBuilder().build(new MockConnection(), { ids: ["id"], entity: "users" }),
    ).toBeInstanceOf(CRUDRepository);
  });

  it("uses the mapped table and columns", () => {
    const statements = new CRUDRepositoryStatements("users", ["id", "name"]);
    expect(DefaultStatementTextRenderer.instance.render(statements.select())).toBe("SELECT id, name FROM users");
    expect(DefaultStatementTextRenderer.instance.render(statements.insert([{ name: "id", value: 1 }]))).toBe(
      "INSERT INTO users (id) VALUES ($id)",
    );
    expect(DefaultStatementTextRenderer.instance.render(statements.update([{ name: "name", value: "a" }]))).toBe(
      "UPDATE users SET name = $name",
    );
    expect(
      DefaultStatementTextRenderer.instance.render(statements.update([{ name: "name", value: "a" }], new Where(new Operator.Equals("id", 1)))),
    ).toBe("UPDATE users SET name = $name WHERE id = $id");
    expect(DefaultStatementTextRenderer.instance.render(statements.delete(new Where(new Operator.Equals("id", 1))))).toBe(
      "DELETE FROM users WHERE id = $id",
    );
    expect(UpdateStatement.parameterList([{ name: "name", value: "a" }])).toEqual([
      { name: "name", value: "a" },
    ]);
    expect(DefaultStatementTextRenderer.instance.render(statements.delete())).toBe("DELETE FROM users");
    const inserted = statements.selectInsertedId("id");
    expect(inserted.table).toBe("users");
    expect(inserted.column).toBe("id");
    expect(() => DefaultStatementTextRenderer.instance.render(inserted)).toThrow("provider must implement SelectInsertIdStatement");
  });
});
