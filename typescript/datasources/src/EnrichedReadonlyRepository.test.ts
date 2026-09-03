import { describe, expect, it } from "vitest";
import { collectAsync } from "./collectAsync.ts";
import { EnrichedReadonlyRepository } from "./EnrichedReadonlyRepository.ts";
import { ListReadonlyRepository } from "./tests/listReadonlyRepository.ts";

class Parent {
  id = 0;
  type_id = 0;
  type_name = "";
  children: Child[] = [];
}

class TypeRow {
  id = 0;
  name = "";
}

class Child {
  parent_id = 0;
  label = "";
}

const parent = (id: number, type_id: number): Parent =>
  Object.assign(new Parent(), { id, type_id });

describe("EnrichedReadonlyRepository", () => {
  it("fills lookup select and leaves missing default", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10), parent(2, 99)]),
    );
    repo.addEnriched(
      "type_name",
      "type_id",
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "person" }),
      ]),
      "id",
      "name",
    );
    const items = await collectAsync(repo.get());
    expect(items[0]?.type_name).toBe("person");
    expect(items[1]?.type_name).toBe("");
  });

  it("fills lists and getSingle is enriched", async () => {
    const repo = new EnrichedReadonlyRepository(
      new ListReadonlyRepository([parent(1, 10)]),
    );
    repo.addEnriched(
      "type_name",
      "type_id",
      new ListReadonlyRepository([
        Object.assign(new TypeRow(), { id: 10, name: "org" }),
      ]),
      "id",
      "name",
    );
    repo.addEnriched(
      "children",
      "id",
      new ListReadonlyRepository([
        Object.assign(new Child(), { parent_id: 1, label: "a" }),
        Object.assign(new Child(), { parent_id: 1, label: "b" }),
        Object.assign(new Child(), { parent_id: 2, label: "x" }),
      ]),
      "parent_id",
      undefined,
      true,
    );
    const item = await repo.getSingle();
    expect(item?.type_name).toBe("org");
    expect(item?.children.map((child) => child.label)).toEqual(["a", "b"]);
  });
});
