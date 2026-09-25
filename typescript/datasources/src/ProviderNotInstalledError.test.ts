import { describe, expect, it } from "vitest";
import { ProviderNotInstalledError } from "./index.ts";

describe("ProviderNotInstalledError", () => {
  it("reports a missing provider", () => {
    const error = new ProviderNotInstalledError("mysql");
    expect(error.name).toBe("ProviderNotInstalledError");
    expect(error.provider).toBe("mysql");
    expect(error.message).toContain("mysql");
  });
});
