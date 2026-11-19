import { describe, expect, it } from "vitest";
import { graphLine, regionLine, transcript, treeLine } from "./format.ts";

describe("format", () => {
  it("prints a nested tree, a region path, and graph edges", () => {
    const apple = { name: "apple", children: [] };
    const fruit = { name: "fruit", children: [apple] };
    const food = { name: "food", children: [fruit] };
    expect(treeLine(food)).toBe("food fruit apple");
    expect(treeLine(apple)).toBe("apple");
    const child = { regionId: 1, categoryId: 2, name: "coast", children: [] };
    const root = { regionId: 1, categoryId: 1, name: "west", children: [child] };
    expect(regionLine(root)).toBe("1-1/west 1-2/coast");
    expect(graphLine({ value: "core", edges: [{ value: "potion" }, { value: "blade" }] })).toBe("core blade,potion");
    expect(graphLine({ value: "blade", edges: [] })).toBe("blade");
    expect(transcript([food], [root], [{ value: "core", edges: [{ value: "blade" }] }])).toEqual([
      "graph core blade",
      "region 1-1/west 1-2/coast",
      "tree food fruit apple",
    ]);
  });
});
