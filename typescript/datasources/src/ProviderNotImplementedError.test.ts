import { describe, expect, it } from "vitest";
import { ProviderNotImplementedError } from "./index.ts";

describe("ProviderNotImplementedError", () => {
  it("reports an unimplemented provider", () => {
    const error = new ProviderNotImplementedError("oracle");
    expect(error.name).toBe("ProviderNotImplementedError");
    expect(error.provider).toBe("oracle");
    expect(error.message).toContain("oracle");
  });
});
