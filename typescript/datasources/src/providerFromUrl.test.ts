import { describe, expect, it } from "vitest";
import { providerFromUrl } from "./index.ts";

describe("providerFromUrl", () => {
  it("reads the scheme", () => {
    expect(providerFromUrl("")).toBeUndefined();
    expect(providerFromUrl("not-a-url")).toBeUndefined();
    expect(providerFromUrl("postgresql://db")).toBe("postgresql");
    expect(providerFromUrl("sqlite3://x")).toBe("sqlite3");
    expect(providerFromUrl("mssql://x")).toBe("mssql");
    expect(providerFromUrl("postgres://db")).toBe("postgres");
    expect(providerFromUrl("sqlserver://x")).toBe("sqlserver");
    expect(providerFromUrl("json://")).toBe("json");
  });
});
