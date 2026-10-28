import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "../io.ts";
import type { Flags, GenerateCommand } from "../parse.ts";
import type { Deps } from "../run.ts";
import type { Prompter } from "../setup.ts";
import { applyMigrateUp } from "./migrateUp.ts";
import { register } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";

register("sqlite", new SqliteDriver());

const silent: Prompter = {
  select: async ({ default: def }) => def,
  checkbox: async ({ choices }) => choices.filter((choice) => choice.checked).map((choice) => choice.value),
  input: async ({ default: def }) => def,
  confirm: async ({ default: def }) => def,
};

const mem = (files: Record<string, string> = {}): { files: Record<string, string>; fs: Fs } => {
  const store = { ...files };
  return {
    files: store,
    fs: {
      readFile: async (path) => {
        const text = store[path];
        if (text === undefined) throw new Error("ENOENT");
        return text;
      },
      writeFile: async (path, text) => {
        store[path] = text;
      },
      readdir: async () => [],
    },
  };
};

const deps = (fs: Fs): Deps & { logs: string[]; ran: string[] } => {
  const logs: string[] = [];
  const ran: string[] = [];
  return {
    fs,
    run: async (command, args) => {
      ran.push(`${command} ${args.join(" ")}`);
    },
    log: (line) => logs.push(line),
    err: () => undefined,
    prompt: silent,
    logs,
    ran,
  };
};

const flags = (cwd = "app"): Flags => ({ cwd, languages: [], datasources: [], dryRun: false, yes: false });

const command = (extra: Partial<GenerateCommand> = {}): GenerateCommand => ({
  kind: "generate",
  target: "all",
  dialect: "typescript",
  provider: "deterministic",
  input: "deterministic",
  output: "generated",
  migrations: true,
  migrate: true,
  storedProcedures: false,
  migrationsRoot: "sql",
  ...extra,
});

const spec = (settings = "settings:\n  backend:\n    datasources: sqlite, sqlite,,json\n"): Record<string, string> => ({
  [join("app", "deterministic", "types.yaml")]: "version: 1.0.0\ntypes: []\n",
  [join("app", "deterministic", "settings.yaml")]: settings,
});

describe("applyMigrateUp", () => {
  it("skips when the database is missing, in memory, or not SQL", async () => {
    const missing = deps(mem(spec()).fs);
    await applyMigrateUp(command(), flags(), missing, {});
    expect(missing.logs[0]).toMatch(/set PROVIDER and DATASOURCE_URL/);
    const bare = deps(mem({ ...spec(), [join("app", ".env")]: "DATASOURCE_URL=./app.sqlite\n" }).fs);
    await applyMigrateUp(command(), flags(), bare, {});
    expect(bare.logs[0]).toMatch(/set PROVIDER and DATASOURCE_URL/);
    const noUrl = deps(mem({ ...spec(), [join("app", ".env")]: "PROVIDER=sqlite\n" }).fs);
    await applyMigrateUp(command(), flags(), noUrl, {});
    expect(noUrl.logs[0]).toMatch(/set PROVIDER and DATASOURCE_URL/);
    const memory = deps(mem({ ...spec(), [join("app", ".env")]: "PROVIDER=sqlite\nDATASOURCE_URL=:memory:\n" }).fs);
    await applyMigrateUp(command(), flags(), memory, {});
    expect(memory.logs[0]).toMatch(/in-memory/);
    const json = deps(mem({ ...spec(), [join("app", ".env")]: "PROVIDER=json\nDATASOURCE_URL=json://files\n" }).fs);
    await applyMigrateUp(command(), flags(), json, {});
    expect(json.logs[0]).toMatch(/json has no SQL migrations/);
  });

  it("uses the URL scheme, aliases, and --migrations-root", async () => {
    const fromUrl = deps(
      mem({
        ...spec("settings:\n  backend:\n    datasources: postgres\n"),
        [join("app", ".env")]: "DATASOURCE_URL=postgres://localhost/db\n",
      }).fs,
    );
    await applyMigrateUp(command(), flags(), fromUrl, {});
    expect(fromUrl.ran).toEqual([
      "npx migrate-setup --provider postgres --connection postgres://localhost/db",
      "npx migrate-up --provider postgres --connection postgres://localhost/db --migrations-root sql",
    ]);
    const aliased = deps(
      mem({
        ...spec("settings:\n  backend:\n    datasources: postgres\n"),
        [join("app", ".env")]: "PROVIDER=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n",
      }).fs,
    );
    await applyMigrateUp(command(), flags(), aliased, {
      PROVIDER: "PostgreSQL",
      DATASOURCE_URL: "postgresql://localhost/db",
    });
    expect(aliased.ran).toEqual([
      "npx migrate-setup --provider postgres --connection postgresql://localhost/db",
      "npx migrate-up --provider postgres --connection postgresql://localhost/db --migrations-root sql",
    ]);
    const absolute = deps(
      mem({
        [join("/tmp/spec", "types.yaml")]: "version: 1.0.0\ntypes: []\n",
        [join("/tmp/spec", "settings.yaml")]: "settings:\n  backend:\n    datasources: sqlite\n",
      }).fs,
    );
    await applyMigrateUp(command({ input: "/tmp/spec" }), flags(), absolute, {
      PROVIDER: "sqlite",
      DATASOURCE_URL: "sqlite:///./app.sqlite",
    });
    expect(absolute.ran[1]).toContain("--migrations-root sql");
  });

  it("rejects a provider that is not installed", async () => {
    const io = deps(
      mem({
        ...spec("settings:\n  backend:\n    datasources: sqlite\n"),
        [join("app", ".env")]: "INSTALLED_PROVIDERS=json\n",
      }).fs,
    );
    await expect(
      applyMigrateUp(command(), flags(), io, { PROVIDER: "sqlite", DATASOURCE_URL: "sqlite:///./app.sqlite" }),
    ).rejects.toThrow(/INSTALLED_PROVIDERS \(none\)/);
  });

});
