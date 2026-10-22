import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "./io.ts";
import { defaultDatasourceYaml } from "./generate/loadSpec.ts";
import { pluginFileName, run, type Deps } from "./run.ts";
import { starterTypesYaml, type Prompter } from "./setup.ts";

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
  it("names rust plugins per platform", () => {
    expect(pluginFileName("datasources-sqlite", "win32")).toBe("datasources_sqlite.dll");
    expect(pluginFileName("datasources-sqlite", "darwin")).toBe("libdatasources_sqlite.dylib");
    expect(pluginFileName("datasources-sqlite", "linux")).toBe("libdatasources_sqlite.so");
  });

  it("prints help", async () => {
    const io = deps(mem([]).fs);
    expect(await run(["--help"], io, cwd)).toBe(0);
    expect(io.logs[0]).toMatch(/Usage:/);
  });

  it("sets up with --yes and installs missing providers", async () => {
    const { files, fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({ dependencies: {} }),
      [join(cwd, ".env.example")]: "INSTALLED_PROVIDERS=sqlite\nCUSTOM=keep\n",
    });
    const io = deps(fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--ecosystem", "ts"], io)).toBe(0);
    expect(files[join(cwd, ".env")]).toMatch(/INSTALLED_PROVIDERS=sqlite/);
    expect(files[join(cwd, ".env")]).toMatch(/DATASOURCE_URL=sqlite:\/\/\/:memory:/);
    expect(files[join(cwd, ".env")]).toMatch(/CUSTOM=keep/);
    expect(io.ran[0]).toMatch(/npm install/);
    expect(io.ran[0]).toMatch(/@deterministic-code\/datasources-sqlite/);
    expect(io.ran[0]).not.toMatch(/better-sqlite3/);
    expect(io.ran).toContain("npm install @deterministic-code/migraters");
    expect(io.ran).toContain(
      "npx --yes --package @deterministic-code/migraters migrate-init --language typescript --dialect sqlite",
    );
    expect(files[join(cwd, "deterministic", "types.yaml")]).toBeUndefined();
  });

  it("inits migraters for checked SQL dialects and skips json-only", async () => {
    const sql = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({ dependencies: {} }),
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite,postgres\nDATASOURCE_URL=sqlite:///:memory:\n",
    });
    const sqlIo = deps(sql.fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--ecosystem", "ts"], sqlIo)).toBe(0);
    expect(sqlIo.ran).toContain("npm install @deterministic-code/migraters");
    expect(sqlIo.ran).toContain(
      "npx --yes --package @deterministic-code/migraters migrate-init --language typescript --dialect sqlite --dialect postgres",
    );
    const json = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({ dependencies: { "@deterministic-code/datasources-json": "*" } }),
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=json\nDATASOURCE_URL=json://\n",
    });
    const jsonIo = deps(json.fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--ecosystem", "ts"], jsonIo)).toBe(0);
    expect(jsonIo.ran.some((line) => line.includes("migraters") || line.includes("migrate-init"))).toBe(false);
    const dry = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({ dependencies: { "@deterministic-code/datasources-sqlite": "*" } }),
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n",
    });
    const dryIo = deps(dry.fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--dry-run"], dryIo)).toBe(0);
    expect(dryIo.ran).toEqual([]);
    expect(dryIo.logs.some((line) => line.includes("migrate-init --language typescript --dialect sqlite"))).toBe(true);
    const addIo = deps(mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({ dependencies: { "@deterministic-code/datasources-sqlite": "*" } }),
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\n",
    }).fs);
    expect(await run(["add", "postgres", "--cwd", cwd, "--ecosystem", "ts"], addIo)).toBe(0);
    expect(addIo.ran).toContain(
      "npx --yes --package @deterministic-code/migraters migrate-init --language typescript --dialect sqlite --dialect postgres",
    );
  });

  it("inits migraters when csharp or rust adds SQL dialects", async () => {
    const init = (language: string) =>
      `npx --yes --package @deterministic-code/migraters migrate-init --language ${language} --dialect sqlite --dialect postgres`;
    const csharp = mem(["App.csproj"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const csharpIo = deps(csharp.fs);
    expect(await run(["add", "sqlite", "postgres", "--cwd", cwd, "--ecosystem", "csharp"], csharpIo)).toBe(0);
    expect(csharpIo.ran).toContain("npm install @deterministic-code/migraters");
    expect(csharpIo.ran).toContain(init("csharp"));
    const rust = mem(["Cargo.toml"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const rustIo = deps(rust.fs);
    expect(await run(["add", "sqlite", "postgres", "--cwd", cwd, "--ecosystem", "rust"], rustIo)).toBe(0);
    expect(rustIo.ran).toContain("npm install @deterministic-code/migraters");
    expect(rustIo.ran).toContain(init("rust"));
    const json = mem(["App.csproj"], {
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=json\nDATASOURCE_URL=json://\n",
    });
    const jsonIo = deps(json.fs);
    expect(await run(["add", "json", "--cwd", cwd, "--ecosystem", "csharp"], jsonIo)).toBe(0);
    expect(jsonIo.ran.some((line) => line.includes("migraters") || line.includes("migrate-init"))).toBe(false);
    const setup = mem(["App.csproj"], {
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n",
    });
    const setupIo = deps(setup.fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--ecosystem", "csharp"], setupIo)).toBe(0);
    expect(setupIo.ran).toContain(
      "npx --yes --package @deterministic-code/migraters migrate-init --language csharp --dialect sqlite",
    );
    const csharpSetup = mem(["App.csproj"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const csharpSetupIo = deps(csharpSetup.fs);
    expect(
      await run(["setup", "--yes", "--cwd", cwd, "--ecosystem", "csharp", "sqlite", "postgres"], csharpSetupIo),
    ).toBe(0);
    expect(csharpSetup.files[join(cwd, ".env")]).toMatch(/INSTALLED_PROVIDERS=sqlite,postgres/);
    expect(csharpSetupIo.ran).toContain(init("csharp"));
    const rustSetup = mem(["Cargo.toml"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const rustSetupIo = deps(rustSetup.fs);
    expect(
      await run(["setup", "--yes", "--cwd", cwd, "--ecosystem", "rust", "sqlite", "postgres"], rustSetupIo),
    ).toBe(0);
    expect(rustSetup.files[join(cwd, ".env")]).toMatch(/INSTALLED_PROVIDERS=sqlite,postgres/);
    expect(rustSetupIo.ran).toContain(init("rust"));
  });

  it("runs interactive setup and diffs add/remove", async () => {
    const { fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@deterministic-code/datasources-json": "*", "@deterministic-code/datasources-sqlite": "*" },
      }),
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=json,sqlite\nDATASOURCE_URL=json://\n",
    });
    const io = deps(fs, {
      prompt: {
        select: async () => "memory",
        checkbox: async () => ["memory"],
        input: async ({ default: def }) => def,
        confirm: async ({ default: def }) => def,
      },
    });
    expect(await run(["setup", "--cwd", cwd], io)).toBe(0);
    expect(io.ran.some((line) => line.includes("uninstall") && line.includes("json"))).toBe(true);
    expect(io.ran.some((line) => line.includes("install") && line.includes("memory"))).toBe(true);
  });

  it("adds and removes rust crates and builds the plugin", async () => {
    const toml = `datasources-json = { version = "0.1.0" }\n`;
    const { files, fs } = mem(["Cargo.toml"], {
      [join(cwd, "Cargo.toml")]: toml,
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=json\n",
    });
    const io = deps(fs);
    expect(await run(["add", "sqlite", "--cwd", cwd, "--ecosystem", "rust"], io)).toBe(0);
    expect(files[join(cwd, "Cargo.toml")]).toBe(toml);
    expect(files[join(cwd, ".env")]).toMatch(/INSTALLED_PROVIDERS=json,sqlite/);
    expect(io.ran).toContain("cargo add datasources");
    expect(io.ran).toContain("cargo add datasources-sqlite");
    expect(io.ran).toContain("cargo build -p datasources-sqlite");
    expect(await run(["remove", "json", "--cwd", cwd, "--ecosystem", "rust"], io)).toBe(0);
    expect(io.ran).toContain("cargo remove datasources-json");
    const dry = deps(fs);
    expect(await run(["remove", "sqlite", "--cwd", cwd, "--dry-run"], dry)).toBe(0);
    expect(dry.ran).toEqual([]);
    expect(dry.logs.some((line) => line.includes("cargo remove datasources-sqlite"))).toBe(true);
    expect(await run(["add", "json", "--cwd", cwd], deps(mem(["Cargo.toml"]).fs))).toBe(0);
  });

  it("dry-runs csharp without writing env", async () => {
    const { files, fs } = mem(["App.csproj"], {
      [join(cwd, "App.csproj")]: `<PackageReference Include="Datasources.Json" />`,
    });
    const io = deps(fs);
    expect(
      await run(["add", "xml", "--cwd", cwd, "--dry-run"], io),
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

  it("scaffolds a deterministic directory when setup confirms", async () => {
    const { files, fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@deterministic-code/datasources-sqlite": "*" },
      }),
    });
    const io = deps(fs);
    expect(await run(["setup", "--cwd", cwd], io)).toBe(0);
    expect(files[join(cwd, "deterministic", "types.yaml")]).toBe(starterTypesYaml);
    expect(files[join(cwd, "deterministic", "datasource.yaml")]).toBe(defaultDatasourceYaml);
  });

  it("writes types.yaml without replacing an existing datasource.yaml", async () => {
    const kept = "version: 1.0.0\ntypes: []\n";
    const { files, fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@deterministic-code/datasources-sqlite": "*" },
      }),
      [join(cwd, "deterministic", "datasource.yaml")]: kept,
    });
    const io = deps(fs);
    expect(await run(["setup", "--cwd", cwd], io)).toBe(0);
    expect(files[join(cwd, "deterministic", "types.yaml")]).toBe(starterTypesYaml);
    expect(files[join(cwd, "deterministic", "datasource.yaml")]).toBe(kept);
  });

  it("leaves an existing types.yaml alone", async () => {
    const kept = "version: 1.0.0\ntypes: []\n";
    const { files, fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@deterministic-code/datasources-sqlite": "*" },
      }),
      [join(cwd, "deterministic", "types.yaml")]: kept,
    });
    const io = deps(fs, {
      prompt: {
        ...silentPrompt,
        confirm: async () => {
          throw new Error("should not ask to scaffold");
        },
      },
    });
    expect(await run(["setup", "--cwd", cwd], io)).toBe(0);
    expect(files[join(cwd, "deterministic", "types.yaml")]).toBe(kept);
    expect(files[join(cwd, "deterministic", "datasource.yaml")]).toBeUndefined();
  });

  it("logs scaffold paths on dry-run without writing them", async () => {
    const { files, fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@deterministic-code/datasources-sqlite": "*" },
      }),
    });
    const io = deps(fs);
    expect(await run(["setup", "--cwd", cwd, "--dry-run"], io)).toBe(0);
    expect(io.logs).toContain(join(cwd, "deterministic", "types.yaml"));
    expect(io.logs).toContain(join(cwd, "deterministic", "datasource.yaml"));
    expect(files[join(cwd, "deterministic", "types.yaml")]).toBeUndefined();
    expect(files[join(cwd, "deterministic", "datasource.yaml")]).toBeUndefined();
  });

  it("skips package commands when the selection is already installed", async () => {
    const { fs } = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({
        dependencies: { "@deterministic-code/datasources-sqlite": "*" },
      }),
    });
    const io = deps(fs);
    expect(await run(["setup", "--yes", "--cwd", cwd], io)).toBe(0);
    expect(io.ran).toEqual([
      "npm install @deterministic-code/migraters",
      "npx --yes --package @deterministic-code/migraters migrate-init --language typescript --dialect sqlite",
    ]);
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

  it("copies a built rust plugin", async () => {
    const root = await mkdtemp(join(tmpdir(), "datasources-cli-"));
    const app = join(root, "app");
    const library = join(app, "target", "debug", "datasources_sqlite.dll");
    await mkdir(join(app, "target", "debug"), { recursive: true });
    await writeFile(library, "plugin");
    const platform = Object.getOwnPropertyDescriptor(process, "platform");
    Object.defineProperty(process, "platform", { value: "win32" });
    try {
      const { fs } = mem(["Cargo.toml"], { [join(app, "Cargo.toml")]: "datasources-json = { version = \"0.1.0\" }\n" });
      const io = deps(fs);
      expect(await run(["add", "sqlite", "--cwd", app, "--ecosystem", "rust"], io)).toBe(0);
      expect(io.logs.some((line) => line.includes("datasources_sqlite.dll"))).toBe(true);
      Object.defineProperty(process, "platform", { value: "linux" });
      expect(await run(["add", "sqlite", "--cwd", app, "--ecosystem", "rust"], deps(fs))).toBe(0);
    } finally {
      if (platform) Object.defineProperty(process, "platform", platform);
      await rm(root, { recursive: true, force: true });
    }
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
