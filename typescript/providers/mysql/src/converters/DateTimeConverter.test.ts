import { describe, expect, it } from "vitest";
import { DateTimeConverter } from "./DateTimeConverter.ts";

describe("DateTimeConverter", () => {
  it("maps datetime", () => {
    const converter = new DateTimeConverter();
    const date = new Date("2020-01-02T03:04:05.000Z");
    expect(converter.convert("2020-01-02T03:04:05.000Z")).toEqual(date);
    expect(converter.convert("2020-01-02 03:04:05.000")).toEqual(date);
    expect(converter.convert("2020-01-02T03:04:05.000")).toEqual(date);
    expect(converter.convert("2020-01-02T14:04:05.000+11:00")).toEqual(date);
    expect(converter.convert("nope")).toBe("nope");
    expect(converter.convert(date)).toBe(date);
    expect(converter.reverseConvert(date)).toBe("2020-01-02 03:04:05.000");
    expect(converter.reverseConvert("2020-01-02T03:04:05.000Z")).toBe("2020-01-02 03:04:05.000");
    expect(converter.reverseConvert("x")).toBe("x");
  });
});
