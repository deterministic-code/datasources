import { describe, expect, it } from "vitest";
import { RowTypeConverter } from "../converters/RowTypeConverter.ts";
import { RowTypeMapper } from "./RowTypeMapper.ts";

describe("RowTypeMapper", () => {
  it("maps a flat row to T and back", () => {
    const mapper = new RowTypeMapper<{ id: number; name: string }>(new RowTypeConverter());
    const row = { id: 1, name: "ada" };
    expect(mapper.map(row)).toEqual(row);
    expect(mapper.reverseMap(row)).toEqual(row);
  });
});
