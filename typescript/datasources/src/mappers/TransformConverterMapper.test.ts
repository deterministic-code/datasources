import { describe, expect, it } from "vitest";
import { TypeConverter } from "../converters/TypeConverter.ts";
import { TransformConverterMapper } from "./TransformConverterMapper.ts";

class Wrap extends TypeConverter<string, string> {
  convert = (from: string) => `[${from}]`;
  reverseConvert = (to: string) => to.replace(/^\[|\]$/g, "");
}

describe("TransformConverterMapper", () => {
  it("converts both ways without a filter", () => {
    const mapper = new TransformConverterMapper(new Wrap());
    expect(mapper.map("ab")).toBe("[ab]");
    expect(mapper.reverseMap("[ab]")).toBe("ab");
  });

  it("converts when the filter is true", () => {
    const mapper = new TransformConverterMapper(new Wrap(), (s) => s.includes("ab"));
    expect(mapper.map("ab")).toBe("[ab]");
    expect(mapper.reverseMap("[ab]")).toBe("ab");
  });

  it("is identity when the filter is false", () => {
    const mapper = new TransformConverterMapper(new Wrap(), (s) => s.includes("ab"));
    expect(mapper.map("a")).toBe("a");
    expect(mapper.reverseMap("[a]")).toBe("[a]");
  });
});
