import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "./io.ts";
import { defaultDatasourceYaml } from "./generate/loadSpec.ts";
import { run, type Deps } from "./run.ts";
import { missingProject } from "./detect.ts";
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
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "ts"], io)).toBe(0);
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
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "ts"], sqlIo)).toBe(0);
    expect(sqlIo.ran).toContain("npm install @deterministic-code/migraters");
    expect(sqlIo.ran).toContain(
      "npx --yes --package @deterministic-code/migraters migrate-init --language typescript --dialect sqlite --dialect postgres",
    );
    const json = mem(["package.json"], {
      [join(cwd, "package.json")]: JSON.stringify({ dependencies: { "@deterministic-code/datasources-json": "*" } }),
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=json\nDATASOURCE_URL=json://\n",
    });
    const jsonIo = deps(json.fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "ts"], jsonIo)).toBe(0);
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
    expect(await run(["add", "postgres", "--cwd", cwd, "--languages", "ts"], addIo)).toBe(0);
    expect(addIo.ran).toContain(
      "npx --yes --package @deterministic-code/migraters migrate-init --language typescript --dialect sqlite --dialect postgres",
    );
  });

  it("inits migraters when csharp or rust adds SQL dialects", async () => {
    const init = (language: string) =>
      `npx --yes --package @deterministic-code/migraters migrate-init --language ${language} --dialect sqlite --dialect postgres`;
    const csharp = mem(["App.csproj"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const csharpIo = deps(csharp.fs);
    expect(await run(["add", "sqlite", "postgres", "--cwd", cwd, "--languages", "csharp"], csharpIo)).toBe(0);
    expect(csharpIo.ran).toContain(
      "npm install @deterministic-code/datasources @deterministic-code/datasources-sqlite @deterministic-code/datasources-postgres",
    );
    expect(csharpIo.ran).toContain("npm install @deterministic-code/migraters");
    expect(csharpIo.ran).toContain(init("csharp"));
    const rust = mem(["Cargo.toml"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const rustIo = deps(rust.fs);
    expect(await run(["add", "sqlite", "postgres", "--cwd", cwd, "--languages", "rust"], rustIo)).toBe(0);
    expect(rustIo.ran).toContain(
      "npm install @deterministic-code/datasources @deterministic-code/datasources-sqlite @deterministic-code/datasources-postgres",
    );
    expect(rustIo.ran).toContain("npm install @deterministic-code/migraters");
    expect(rustIo.ran).toContain(init("rust"));
    const json = mem(["App.csproj"], {
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=json\nDATASOURCE_URL=json://\n",
    });
    const jsonIo = deps(json.fs);
    expect(await run(["add", "json", "--cwd", cwd, "--languages", "csharp"], jsonIo)).toBe(0);
    expect(jsonIo.ran.some((line) => line.includes("migraters") || line.includes("migrate-init"))).toBe(false);
    const setup = mem(["App.csproj"], {
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n",
    });
    const setupIo = deps(setup.fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "csharp"], setupIo)).toBe(0);
    expect(setupIo.ran).toContain(
      "npx --yes --package @deterministic-code/migraters migrate-init --language csharp --dialect sqlite",
    );
    const csharpSetup = mem(["App.csproj"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const csharpSetupIo = deps(csharpSetup.fs);
    expect(
      await run(
        ["setup", "--yes", "--cwd", cwd, "--languages", "csharp", "--datasources", "sqlite", "postgres"],
        csharpSetupIo,
      ),
    ).toBe(0);
    expect(csharpSetup.files[join(cwd, ".env")]).toMatch(/INSTALLED_PROVIDERS=sqlite,postgres/);
    expect(csharpSetupIo.ran).toContain(init("csharp"));
    const rustSetup = mem(["Cargo.toml"], { [join(cwd, ".env")]: "INSTALLED_PROVIDERS=\n" });
    const rustSetupIo = deps(rustSetup.fs);
    expect(
      await run(
        ["setup", "--yes", "--cwd", cwd, "--languages", "rust", "--datasources", "sqlite", "postgres"],
        rustSetupIo,
      ),
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
    let asked = 0;
    const mixed = deps(mem(["package.json", "Cargo.toml"]).fs, {
      prompt: {
        select: async () => "",
        checkbox: async () => (++asked === 1 ? [] : ["sqlite"]),
        input: async ({ default: def }) => def,
        confirm: async () => false,
      },
    });
    expect(await run(["setup", "--cwd", cwd], mixed)).toBe(0);
    expect(mixed.ran.some((line) => line.includes("npm install"))).toBe(true);
    expect(mixed.ran.some((line) => line.includes("cargo add"))).toBe(true);
  });

  it("adds and removes rust crates and writes the bind file", async () => {
    const toml = `datasources-json = { version = "0.1.0" }\n`;
    const { files, fs } = mem(["Cargo.toml"], {
      [join(cwd, "Cargo.toml")]: toml,
      [join(cwd, ".env")]: "INSTALLED_PROVIDERS=json\n",
    });
    const io = deps(fs);
    expect(await run(["add", "sqlite", "--cwd", cwd, "--languages", "rust"], io)).toBe(0);
    expect(files[join(cwd, "Cargo.toml")]).toBe(toml);
    expect(files[join(cwd, ".env")]).toMatch(/INSTALLED_PROVIDERS=json,sqlite/);
    expect(io.ran).toContain("cargo add datasources");
    expect(io.ran).toContain("cargo add datasources-sqlite");
    expect(io.ran).toContain(
      "npm install @deterministic-code/datasources @deterministic-code/datasources-sqlite",
    );
    expect(files[join(cwd, "datasources_providers.rs")]).toBe(
      "use datasources_json as _;\nuse datasources_sqlite as _;\n",
    );
    files[join(cwd, "Cargo.toml")] =
      `datasources-json = { version = "0.1.0" }\ndatasources-sqlite = { version = "0.1.0" }\n`;
    expect(await run(["remove", "json", "--cwd", cwd, "--languages", "rust"], io)).toBe(0);
    expect(io.ran).toContain("cargo remove datasources-json");
    expect(files[join(cwd, "datasources_providers.rs")]).toBe("use datasources_sqlite as _;\n");
    const dry = deps(fs);
    expect(await run(["remove", "sqlite", "--cwd", cwd, "--dry-run"], dry)).toBe(0);
    expect(dry.ran).toEqual([]);
    expect(dry.logs.some((line) => line.includes("cargo remove datasources-sqlite"))).toBe(true);
    expect(dry.logs.some((line) => line.includes("npm uninstall @deterministic-code/datasources-sqlite"))).toBe(true);
    expect(await run(["add", "json", "--cwd", cwd], deps(mem(["Cargo.toml"]).fs))).toBe(0);
  });

  it("stops when a requested language has no project", async () => {
    const empty = mem([]);
    const csharp = deps(empty.fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "csharp"], csharp)).toBe(1);
    expect(csharp.errors).toEqual([missingProject("csharp")]);
    expect(csharp.ran).toEqual([]);
    expect(empty.files[join(cwd, ".env")]).toBeUndefined();
    const rust = deps(mem([]).fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "rust"], rust)).toBe(1);
    expect(rust.errors).toEqual([missingProject("rust")]);
    const ts = deps(mem([]).fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "ts"], ts)).toBe(1);
    expect(ts.errors).toEqual([missingProject("ts")]);
    const both = deps(mem([]).fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "csharp", "rust"], both)).toBe(1);
    expect(both.errors).toEqual([`${missingProject("csharp")}\n${missingProject("rust")}`]);
    expect(both.ran).toEqual([]);
    const dry = deps(mem([]).fs);
    expect(await run(["setup", "--yes", "--dry-run", "--cwd", cwd, "--languages", "csharp"], dry)).toBe(1);
    expect(dry.logs.some((line) => line.includes("dotnet add"))).toBe(false);
    expect(await run(["add", "sqlite", "--cwd", cwd, "--languages", "csharp"], deps(mem([]).fs))).toBe(1);
    const project = deps(mem(["App.csproj"]).fs);
    expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "csharp"], project)).toBe(0);
    expect(project.ran).toContain("dotnet add App.csproj package Datasources");
    const sqliteOnly = deps(
      mem(["App.csproj"], {
        [join(cwd, "App.csproj")]: `<PackageReference Include="Datasources" />`,
      }).fs,
    );
    expect(
      await run(
        ["setup", "--yes", "--cwd", cwd, "--languages", "csharp", "--datasources", "sqlite"],
        sqliteOnly,
      ),
    ).toBe(0);
    expect(sqliteOnly.ran).toContain("dotnet add App.csproj package Datasources.Sqlite");
    expect(sqliteOnly.ran).toContain(
      "npm install @deterministic-code/datasources @deterministic-code/datasources-sqlite",
    );
  });

  it("writes process DATASOURCE_URL into .env", async () => {
    const previous = {
      INSTALLED_PROVIDERS: process.env.INSTALLED_PROVIDERS,
      PROVIDER: process.env.PROVIDER,
      DATASOURCE_URL: process.env.DATASOURCE_URL,
    };
    process.env.INSTALLED_PROVIDERS = "sqlite";
    process.env.PROVIDER = "sqlite";
    process.env.DATASOURCE_URL = "sqlite:///./app.sqlite";
    try {
      const { files, fs } = mem(["App.csproj"]);
      const io = deps(fs);
      expect(await run(["setup", "--yes", "--cwd", cwd, "--languages", "csharp"], io)).toBe(0);
      expect(io.errors).toEqual([]);
      expect(files[join(cwd, ".env")]).toMatch(/INSTALLED_PROVIDERS=sqlite/);
      expect(files[join(cwd, ".env")]).toMatch(/PROVIDER=sqlite/);
      expect(files[join(cwd, ".env")]).toMatch(/DATASOURCE_URL=sqlite:\/\/\/\.\/app\.sqlite/);
    } finally {
      if (previous.INSTALLED_PROVIDERS === undefined) delete process.env.INSTALLED_PROVIDERS;
      else process.env.INSTALLED_PROVIDERS = previous.INSTALLED_PROVIDERS;
      if (previous.PROVIDER === undefined) delete process.env.PROVIDER;
      else process.env.PROVIDER = previous.PROVIDER;
      if (previous.DATASOURCE_URL === undefined) delete process.env.DATASOURCE_URL;
      else process.env.DATASOURCE_URL = previous.DATASOURCE_URL;
    }
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
    expect(await run(["add", "sqlite"], mixed, cwd)).toBe(0);
    expect(mixed.ran.some((line) => line.includes("npm install"))).toBe(true);
    expect(mixed.ran.some((line) => line.includes("cargo add datasources-sqlite"))).toBe(true);
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
