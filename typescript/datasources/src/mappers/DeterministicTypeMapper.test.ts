import { describe, expect, it } from "vitest";
import { DeterministicTypeMapper } from "./DeterministicTypeMapper.ts";

describe("DeterministicTypeMapper", () => {
  it("maps language type names to types.yaml names", () => {
    const types = new DeterministicTypeMapper();
    expect(types.map("boolean")).toBe("boolean");
    expect(types.map("Boolean")).toBe("boolean");
    expect(types.map("number")).toBe("integer");
    expect(types.map("Number")).toBe("integer");
    expect(types.map("bigint")).toBe("biginteger");
    expect(types.map("BigInt")).toBe("biginteger");
    expect(types.map("Date")).toBe("datetime");
    expect(types.map("Uint8Array")).toBe("binary");
    expect(types.map("string")).toBe("");
    expect(types.reverseMap("boolean")).toBe("boolean");
    expect(types.reverseMap("integer")).toBe("number");
    expect(types.reverseMap("biginteger")).toBe("bigint");
    expect(types.reverseMap("datetime")).toBe("Date");
    expect(types.reverseMap("binary")).toBe("Uint8Array");
    expect(types.reverseMap("string")).toBe("");
  });
});
