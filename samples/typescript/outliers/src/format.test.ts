import { describe, expect, it } from "vitest";
import { graphLine, mapError, mapLines, regionLine, transcript, treeLine } from "./format.ts";
import { MappingCell, mappingCell } from "./mappingCell.ts";

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
    expect(
      transcript([food], [root], [{ value: "core", edges: [{ value: "blade" }] }], [
        "map create text_to_integer 10",
      ]),
    ).toEqual([
      "graph core blade",
      "map create text_to_integer 10",
      "region 1-1/west 1-2/coast",
      "tree food fruit apple",
    ]);
    const row = mappingCell({
      id: 1,
      textToString: "ada",
      textToInteger: 10,
      textToDecimal: "10.5",
      textToDatetime: new Date("2020-01-02T03:04:05.000Z"),
      textToBinary: new Uint8Array([1, 2]),
      textToGuid: "550e8400-e29b-41d4-a716-446655440000",
      intToBoolean: true,
      boolToBoolean: true,
      binaryToBinary: "AQI=",
      binaryToGuid: "550e8400-e29b-41d4-a716-446655440000",
      guidToString: "550e8400-e29b-41d4-a716-446655440000",
      guidToGuid: "550e8400-e29b-41d4-a716-446655440000",
    });
    expect(mapLines("create", row)).toContain(
      "map create text_to_guid 550e8400-e29b-41d4-a716-446655440000",
    );
    expect(mapLines("create", row)).toContain("map create text_to_binary 0102");
    expect(mapLines("create", row)).toContain("map create binary_to_binary 0102");
    expect(mapLines("create", row)).toContain("map create int_to_boolean true");
    expect(mapLines("read", mappingCell({ decimalToFloat: 1.23456789012346 }))).toContain(
      "map read decimal_to_float 1.23456789012346",
    );
    expect(mapLines("read", mappingCell({ decimalToFloat: Number.POSITIVE_INFINITY }))).toContain(
      "map read decimal_to_float Infinity",
    );
    expect(mapError("read", '"10.9" is not a long')).toEqual(['map read error "10.9" is not a long']);
    expect(mapLines("create", new MappingCell())).toContain("map create guid_to_guid");
    expect(mapLines("create", new MappingCell())).toContain("map create text_to_string");
  });
});
