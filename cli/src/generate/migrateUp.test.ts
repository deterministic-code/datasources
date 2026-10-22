import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "../io.ts";
import type { Flags, GenerateCommand } from "../parse.ts";
import { run, type Deps } from "../run.ts";
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

const flags = (cwd = "app"): Flags => ({ cwd, dryRun: false, yes: false });

const command = (extra: Partial<GenerateCommand> = {}): GenerateCommand => ({
  kind: "generate",
  target: "all",
  dialect: "typescript",
  provider: "deterministic",
  input: "deterministic",
  output: "generated",
  migrations: true,
  migrate: true,
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

  it("uses the URL scheme, aliases, and an explicit migrations directory", async () => {
    const fromUrl = deps(
      mem({
        ...spec("settings:\n  backend:\n    datasources: postgres\n"),
        [join("app", ".env")]: "DATASOURCE_URL=postgres://localhost/db\n",
      }).fs,
    );
    await applyMigrateUp(command(), flags(), fromUrl, {});
    expect(fromUrl.ran[0]).toBe(
      "npx --yes --package @deterministic-code/migraters migrate-up --provider postgres --connection postgres://localhost/db --migrations-root sql",
    );
    const aliased = deps(
      mem({
        ...spec("settings:\n  backend:\n    datasources: postgres\n"),
        [join("app", ".env")]: "PROVIDER=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n",
      }).fs,
    );
    await applyMigrateUp(command({ migrationsPath: "db/changes" }), flags(), aliased, {
      PROVIDER: "PostgreSQL",
      DATASOURCE_URL: "postgresql://localhost/db",
    });
    expect(aliased.ran[0]).toBe(
      "npx --yes --package @deterministic-code/migraters migrate-up --provider postgres --connection postgresql://localhost/db --migrations-path db/changes",
    );
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
    expect(absolute.ran[0]).toContain("--migrations-root sql");
  });

  it("rejects a provider the spec does not list", async () => {
    const io = deps(mem(spec("settings:\n  backend:\n    datasources: json\n")).fs);
    await expect(
      applyMigrateUp(command(), flags(), io, { PROVIDER: "sqlite", DATASOURCE_URL: "sqlite:///./app.sqlite" }),
    ).rejects.toThrow(/backend.datasources \(none\)/);
  });

  it("generate all honors --migrations-path", async () => {
    const cwd = "app";
    const input = join(cwd, "deterministic");
    const store = mem({
      [join(input, "types.yaml")]: `version: 1.0.0
types:
  - item:
      tags: [datasource_type]
      fields:
        - id:
            type: integer
            is_id: true
`,
      [join(input, "settings.yaml")]: "settings:\n  backend:\n    datasources: sqlite\n",
      [join(cwd, ".env")]: "PROVIDER=sqlite\nDATASOURCE_URL=sqlite:///./app.sqlite\n",
    });
    const io = deps(store.fs);
    const previous = { PROVIDER: process.env.PROVIDER, DATASOURCE_URL: process.env.DATASOURCE_URL };
    delete process.env.PROVIDER;
    delete process.env.DATASOURCE_URL;
    try {
      expect(
        await run(
          ["generate", "all", "--dialect", "typescript", "--cwd", cwd, "--migrations-path", "db/changes"],
          io,
          cwd,
        ),
      ).toBe(0);
    } finally {
      if (previous.PROVIDER === undefined) delete process.env.PROVIDER;
      else process.env.PROVIDER = previous.PROVIDER;
      if (previous.DATASOURCE_URL === undefined) delete process.env.DATASOURCE_URL;
      else process.env.DATASOURCE_URL = previous.DATASOURCE_URL;
    }
    expect(store.files[join(cwd, "db/changes/0001_initial_up.sql")]).toMatch(/CREATE TABLE/i);
    expect(io.ran[0]).toBe(
      "npx --yes --package @deterministic-code/migraters migrate-up --provider sqlite --connection sqlite:///./app.sqlite --migrations-path db/changes",
    );
  });
});
