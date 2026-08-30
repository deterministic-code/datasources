import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "./io.ts";
import { run, type Deps } from "./run.ts";
import type { Prompter } from "./setup.ts";

const cwd = "app";

const mem = (
  entries: string[],
  files: Record<string, string> = {},
): { files: Record<string, string>; fs: Fs } => {
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
      readdir: async () => entries,
    },
  };
};

const silentPrompt: Prompter = {
  select: async ({ default: def }) => def,
  checkbox: async ({ choices }) => choices.filter((c) => c.checked).map((c) => c.value),
  input: async ({ default: def }) => def,
  confirm: async ({ default: def }) => def,
};

const deps = (
  fs: Fs,
  extra: Partial<Deps> & { ran?: string[] } = {},
): Deps & { logs: string[]; errors: string[]; ran: string[] } => {
  const logs: string[] = [];
  const errors: string[] = [];
  const ran = extra.ran ?? [];
  return {
    fs,
    run: extra.run ?? (async (command, args) => {
      ran.push(`${command} ${args.join(" ")}`);
    }),
    log: extra.log ?? ((line) => logs.push(line)),
    err: extra.err ?? ((line) => errors.push(line)),
    prompt: extra.prompt ?? silentPrompt,
    logs,
    errors,
    ran,
  };
};

describe("run", () => {
  it("prints help", async () => {
    const io = deps(mem([]).fs);
    expect(await run(["--help"], io, cwd)).toBe(0);
    expect(io.logs[0]).toMatch(/Usage:/);
  });

  it("sets up with --yes and installs missing providers", async () => {
    const { files, fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({ dependencies: {} }),
      [join(cwd, ".env.example")]: "DATASOURCE_PROVIDER=sqlite\nCUSTOM=keep\n",
    });
    const io = deps(fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--ecosystem", "ts"], io)).toBe(0);
    expect(files[join(cwd, ".env")]).toMatch(/DATASOURCE_PROVIDER=sqlite/);
    expect(files[join(cwd, ".env")]).toMatch(/CUSTOM=keep/);
    expect(io.ran[0]).toMatch(/npm install/);
    expect(io.ran[0]).toMatch(/better-sqlite3/);
  });

  it("runs interactive setup and diffs add/remove", async () => {
    const { fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@datasources/json": "*", "@datasources/sqlite": "*" },
      }),
      [join(cwd, ".env")]: "DATASOURCE_PROVIDER=json\nDATASOURCE_PROVIDERS=json,sqlite\n",
    });
    const io = deps(fs, {
      prompt: {
        select: async () => "memory",
        checkbox: async () => ["memory"],
        input: async ({ default: def }) => def,
        confirm: async () => false,
      },
    });
    expect(await run(["setup", "--cwd", cwd], io)).toBe(0);
    expect(io.ran.some((line) => line.includes("uninstall") && line.includes("json"))).toBe(true);
    expect(io.ran.some((line) => line.includes("install") && line.includes("memory"))).toBe(true);
  });

  it("adds and removes on rust with a feature patch", async () => {
    const { files, fs } = mem(["Cargo.toml"], {
      [join(cwd, "Cargo.toml")]:
        `datasources-register = { version = "0.1.0", features = ["json"] }\n`,
      [join(cwd, ".env")]: "DATASOURCE_PROVIDER=json\n",
    });
    const io = deps(fs);
    expect(await run(["add", "sqlite", "--cwd", cwd, "--ecosystem", "rust"], io)).toBe(0);
    expect(files[join(cwd, "Cargo.toml")]).toMatch(/features = \["json", "sqlite"\]/);
    expect(files[join(cwd, ".env")]).toMatch(/DATASOURCE_PROVIDERS=json,sqlite/);
    expect(await run(["remove", "json", "--cwd", cwd, "--ecosystem", "rust"], io)).toBe(0);
    const dry = deps(fs);
    expect(await run(["remove", "sqlite", "--cwd", cwd, "--dry-run"], dry)).toBe(0);
    expect(dry.logs.some((line) => line.includes("features: (none)"))).toBe(true);
    expect(await run(["add", "json", "--cwd", cwd], deps(mem(["Cargo.toml"]).fs))).toBe(0);
  });

  it("dry-runs csharp without writing cargo features", async () => {
    const { files, fs } = mem(["App.csproj"], {
      [join(cwd, "App.csproj")]: `<PackageReference Include="Datasources.Json" />`,
    });
    const io = deps(fs);
    expect(
      await run(["add", "text", "--cwd", cwd, "--dry-run", "--no-engine"], io),
    ).toBe(0);
    expect(io.ran).toEqual([]);
    expect(io.logs.some((line) => line.includes("dotnet add"))).toBe(true);
    expect(files[join(cwd, ".env")]).toBeUndefined();
    expect(await run(["setup", "--yes", "--dry-run", "--cwd", cwd], deps(fs))).toBe(0);
    expect(files[join(cwd, ".env")]).toBeUndefined();
  });

  it("reads csharp installed packages and rust without a toml", async () => {
    const sln = mem(["App.sln"]);
    expect(await run(["add", "json", "--cwd", cwd, "--dry-run"], deps(sln.fs))).toBe(0);
    const rust = mem(["Cargo.toml"]);
    expect(await run(["add", "json", "--cwd", cwd, "--dry-run"], deps(rust.fs))).toBe(0);
    expect(rust.files[join(cwd, "Cargo.toml")]).toBeUndefined();
  });

  it("skips package commands when the selection is already installed", async () => {
    const { fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@datasources/sqlite": "*" },
      }),
    });
    const io = deps(fs);
    expect(await run(["setup", "--yes", "--cwd", cwd], io)).toBe(0);
    expect(io.ran).toEqual([]);
  });

  it("returns errors for bad usage", async () => {
    const io = deps(mem(["package.json"]).fs);
    expect(await run(["add"], io, cwd)).toBe(1);
    expect(io.errors[0]).toMatch(/Usage: datasources add/);
    expect(await run(["wat"], io, cwd)).toBe(1);
    expect(await run(["add", "sqlite"], deps(mem([]).fs), cwd)).toBe(1);
    const mixed = deps(mem(["package.json", "Cargo.toml"]).fs);
    expect(await run(["add", "sqlite"], mixed, cwd)).toBe(1);
    expect(mixed.errors[0]).toMatch(/Multiple project types/);
  });

  it("maps non-Error throws to a generic message", async () => {
    const io = deps(mem(["package.json"]).fs, {
      run: async () => {
        throw "boom";
      },
    });
    expect(await run(["add", "json", "--cwd", cwd], io)).toBe(1);
    expect(io.errors[0]).toBe("Unknown error");
  });
});
