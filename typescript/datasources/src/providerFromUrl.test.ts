import { describe, expect, it } from "vitest";
import { providerFromUrl } from "./index.ts";

describe("providerFromUrl", () => {
  it("reads the scheme and aliases", () => {
    expect(providerFromUrl("")).toBeUndefined();
    expect(providerFromUrl("not-a-url")).toBeUndefined();
    expect(providerFromUrl("postgresql://db")).toBe("postgres");
    expect(providerFromUrl("sqlite3://x")).toBe("sqlite");
    expect(providerFromUrl("mssql://x")).toBe("sqlserver");
    expect(providerFromUrl("json://")).toBe("json");
  });
});
