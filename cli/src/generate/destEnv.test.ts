import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { destEnv } from "./destEnv.ts";

describe("destEnv", () => {
  const keys = ["INSTALLED_PROVIDERS", "PROVIDER", "DATASOURCE_URL"] as const;
  const previous: Record<string, string | undefined> = {};
  beforeEach(() => {
    for (const key of keys) {
      previous[key] = process.env[key];
      delete process.env[key];
    }
  });
  afterEach(() => {
    for (const key of keys) {
      if (previous[key] === undefined) delete process.env[key];
      else process.env[key] = previous[key];
    }
  });

  it("reads dest .env and falls back when the file is missing", async () => {
    const cwd = "app";
    const files: Record<string, string> = {
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=mysql\n",
    };
    const fs = {
      readFile: async (path: string) => {
        const text = files[path];
        if (text === undefined) throw new Error("ENOENT");
        return text;
      },
    };
    expect(await destEnv(fs, cwd)).toMatchObject({ INSTALLED_PROVIDERS: "mysql" });
    expect(await destEnv(fs, "missing")).toEqual({});
  });
});
