import { mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { DotEnv } from "./dotEnv.ts";

describe("DotEnv", () => {
  it("parses keys and skips comments", () => {
    expect(DotEnv.parse("# hi\n\nPROVIDER=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n")).toEqual({
      PROVIDER: "sqlite",
      DATASOURCE_URL: "sqlite:///:memory:",
    });
    expect(DotEnv.parse("=skip\nKEY=value")).toEqual({ KEY: "value" });
    expect(DotEnv.load("missing.env")).toEqual({});
    const dir = mkdtempSync(join(tmpdir(), "ds-"));
    const path = join(dir, ".env");
    writeFileSync(path, "PROVIDER=sqlite\n");
    expect(DotEnv.load(path)).toEqual({ PROVIDER: "sqlite" });
    rmSync(dir, { recursive: true });
  });
});
