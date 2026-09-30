import { describe, expect, it } from "vitest";
import { DictionaryMapper } from "./mappers/DictionaryMapper.ts";
import { StatementMappings } from "./StatementMappings.ts";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import { Identifier } from "./statements/Identifier.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { Join } from "./statements/Join.ts";
import { Operator } from "./statements/Operator.ts";
import { OrderBy } from "./statements/OrderBy.ts";
import { SelectInsertedIdStatement } from "./statements/SelectInsertedIdStatement.ts";
import { SelectStatement } from "./statements/SelectStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import { Where } from "./statements/Where.ts";
import { WhereText } from "./statements/WhereText.ts";
import type { Cell } from "./converters/Cell.ts";

const mappings = new StatementMappings(
  new DictionaryMapper([
    ["User", "people"],
    ["Role", "roles"],
  ]),
  new DictionaryMapper([
    ["Name", "full_name"],
    ["title", "book.title"],
    ["id", "user_id"],
  ]),
);

const cell = (name: string, value: Cell): Cell =>
  typeof value === "string" ? `${name}:${value.toUpperCase()}` : value;

describe("StatementMappings", () => {
  it("maps a select", () => {
    const select = new SelectStatement({
      distinct: true,
      table: "User",
      fields: ["Name", "title", "scope.title", "alias.other", '"schema.name".id'],
      where: new Where(new Operator.Equals("Name", "Bea")),
      joins: [Join.inner("Role", "User.id", "Role.user_id"), Join.cross("logs")],
      groupBy: ["Name"],
      having: new Where(new Operator.Equals("Name", "Bea")),
      orderBy: [OrderBy.desc("Name")],
      limit: 5,
      offset: 1,
    });
    const mapped = mappings.apply(select, cell) as SelectStatement;
    expect(mapped.table).toBe("people");
    expect(mapped.fields).toEqual([
      "full_name",
      "book.title",
      "book.title",
      "alias.other",
      '"schema.name".user_id',
    ]);
    expect(mapped.distinct).toBe(true);
    expect(mapped.limit).toBe(5);
    expect(mapped.offset).toBe(1);
    expect(mapped.groupBy).toEqual(["full_name"]);
    expect(mapped.orderBy).toEqual([{ column: "full_name", direction: "desc" }]);
    expect((mapped.joins[0]?.on as Operator.Equals).left).toBeInstanceOf(Identifier);
    expect(((mapped.joins[0]?.on as Operator.Equals).left as Identifier).name).toBe("people.user_id");
    expect(mapped.joins[1]).toEqual({ kind: "cross", table: "logs" });
    expect(mapped.getText()).toContain("full_name = $full_name");
    expect(mapped.where?.parameters?.[0]?.value).toBe("Name:BEA");
    expect(mappings.property("full_name")).toBe("Name");
    expect(mappings.property("people.full_name")).toBe("Name");
    expect(mappings.property("other")).toBe("other");
    expect(mappings.property("people.missing")).toBe("people.missing");
  });

  it("maps writes", () => {
    const insert = mappings.apply(new InsertStatement("User", [{ name: "Name", value: "Ada" }]), cell) as InsertStatement;
    expect(insert.getText()).toBe("INSERT INTO people (full_name) VALUES ($full_name)");
    expect(insert.fields[0]?.value).toBe("Ada");

    const update = mappings.apply(
      new UpdateStatement("User", [{ name: "Name", value: "Ada" }], new Where(new Operator.Equals("Name", "Bea"))),
      cell,
    ) as UpdateStatement;
    expect(update.getText()).toBe(
      "UPDATE people SET full_name = $full_name WHERE full_name = $full_name__where",
    );
    expect(update.parameters[0]?.value).toBe("Ada");
    expect(update.parameters[1]).toEqual({ name: "full_name__where", value: "Name:BEA" });

    const deleted = mappings.apply(
      new DeleteStatement("User", new Where(new Operator.Equals("Name", "Bea"))),
      cell,
    ) as DeleteStatement;
    expect(deleted.getText()).toBe("DELETE FROM people WHERE full_name = $full_name");

    const inserted = mappings.apply(new SelectInsertedIdStatement("User", "id"), cell) as SelectInsertedIdStatement;
    expect(inserted.table).toBe("people");
    expect(inserted.column).toBe("user_id");
  });

  it("leaves raw and other statements", () => {
    const rawInsert = new InsertStatement("INSERT INTO t VALUES (1)");
    expect(mappings.apply(rawInsert, cell)).toBe(rawInsert);
    const rawUpdate = new UpdateStatement("x");
    expect(mappings.apply(rawUpdate, cell)).toBe(rawUpdate);
    expect(rawUpdate.parameters).toEqual([]);
    const rawDelete = DeleteStatement.raw("DELETE FROM t");
    expect(mappings.apply(rawDelete, cell)).toBe(rawDelete);
    const bare = new SelectStatement("SELECT 1");
    expect(mappings.apply(bare, cell)).toBe(bare);
    const call = new ExecuteStoredProcedureStatement("CALL save_user()");
    expect(mappings.apply(call, cell)).toBe(call);
    expect(mappings.apply(undefined, cell)).toBeUndefined();
    const text = new WhereText("name = $name");
    expect(mappings.applyWhere(text, cell)).toBe(text);
    const kept = mappings.apply(new DeleteStatement("User", text), cell) as DeleteStatement;
    expect(kept.where).toBe(text);
    expect(kept.getText()).toBe("DELETE FROM people WHERE name = $name");
  });
});
