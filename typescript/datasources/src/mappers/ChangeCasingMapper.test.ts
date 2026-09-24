import { describe, expect, it } from "vitest";
import { ChangeCasingMapper } from "./ChangeCasingMapper.ts";

describe("ChangeCasingMapper", () => {
  it("maps pascal to snake and back", () => {
    const mapper = new ChangeCasingMapper("pascalCase", "snakeCase");
    expect(mapper.map("UserType")).toBe("user_type");
    expect(mapper.map("Id")).toBe("id");
    expect(mapper.reverseMap("user_type")).toBe("UserType");
    expect(mapper.reverseMap("id")).toBe("Id");
  });
});
