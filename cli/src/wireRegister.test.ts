import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "./io.ts";
import { attached, cargoWithCtor, hookContents, hookPath, specFrom, wireRegister } from "./wireRegister.ts";

const tsRegistry = (cwd: string) => join(cwd, "generated", "typescript/repositories/repositoryRegistry.ts");
const rustRegistry = (cwd: string) => join(cwd, "generated", "rust/repositories/mod.rs");

const mem = (entries: string[], files: Record<string, string> = {}): { files: Record<string, string>; fs: Fs } => {
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

describe("wireRegister", () => {
  it("adds ctor to a rust manifest", () => {
    expect(cargoWithCtor("[dependencies]\nfoo = \"1\"\n")).toContain('ctor = "0.4"');
    expect(cargoWithCtor("[package]\nname = \"app\"\n")).toContain("[dependencies]");
    expect(cargoWithCtor('ctor = "0.4"\n[dependencies]\n')).toBeUndefined();
  });

  it("builds a relative typescript import and rust path", () => {
    const hook = join("app", "src", "datasources-register.ts");
    const rustHook = join("app", "src", "datasources_register.rs");
    const out = join("app", "generated");
    expect(hookContents("typescript", hook, out)).toContain("repositoryRegistry.ts");
    expect(hookContents("rust", rustHook, out)).toContain("mod.rs");
    expect(hookContents("rust", rustHook, out)).toContain("repositoryRegistry::Repositories");
    expect(specFrom(hook, join(out, "typescript/repositories/repositoryRegistry.ts"))).toMatch(/repositoryRegistry/);
  });

  it("skips attach when the hook is already referenced", () => {
    expect(attached("typescript", `import "./datasources-register.ts";\n`, join("app", "src", "index.ts"), join("app", "src", "datasources-register.ts"))).toBeUndefined();
    expect(attached("rust", "mod datasources_register;\n", join("app", "src", "lib.rs"), join("app", "src", "datasources_register.rs"))).toBeUndefined();
  });

  it("logs the typescript hook on dry-run", async () => {
    const cwd = "app";
    const { files, fs } = mem(["src"], {
      [join(cwd, "src", "index.ts")]: `export const app = 1;\n`,
      [tsRegistry(cwd)]: "export const register = () => {};\n",
    });
    const logs: string[] = [];
    await wireRegister(cwd, "generated", "typescript", true, { fs, log: (line) => logs.push(line) });
    expect(files[hookPath(cwd, "typescript", true)]).toBeUndefined();
    expect(logs).toContain(hookPath(cwd, "typescript", true));
  });

  it("skips a hook when generated registry is missing", async () => {
    const cwd = "app";
    const { files, fs } = mem(["src"], {
      [join(cwd, "src", "index.ts")]: `export const app = 1;\n`,
    });
    await wireRegister(cwd, "generated", "typescript", false, { fs, log: () => undefined });
    expect(files[hookPath(cwd, "typescript", true)]).toBeUndefined();
    expect(files[join(cwd, "src", "index.ts")]).toBe(`export const app = 1;\n`);
  });

  it("writes and attaches a typescript hook", async () => {
    const cwd = "app";
    const { files, fs } = mem(["package.json", "src"], {
      [join(cwd, "src", "index.ts")]: `export const app = 1;\n`,
      [tsRegistry(cwd)]: "export const register = () => {};\n",
    });
    const logs: string[] = [];
    await wireRegister(cwd, "generated", "typescript", false, { fs, log: (line) => logs.push(line) });
    expect(files[hookPath(cwd, "typescript", true)]).toContain("repositoryRegistry.ts");
    expect(files[join(cwd, "src", "index.ts")]).toContain("datasources-register");
    expect(logs.length).toBe(2);
  });

  it("writes a rust hook when Cargo.toml is missing", async () => {
    const cwd = "app";
    const { files, fs } = mem(["src"], {
      [join(cwd, "src", "lib.rs")]: `pub fn ready() {}\n`,
      [rustRegistry(cwd)]: "pub struct Repositories;\n",
    });
    await wireRegister(cwd, "generated", "rust", false, { fs, log: () => undefined });
    expect(files[hookPath(cwd, "rust", true)]).toContain("#[path");
    expect(files[join(cwd, "Cargo.toml")]).toBeUndefined();
  });

  it("writes and attaches a rust hook", async () => {
    const cwd = "app";
    const { files, fs } = mem(["Cargo.toml", "src"], {
      [join(cwd, "Cargo.toml")]: `[package]\nname = "app"\n\n[dependencies]\ndatasources = "0.1"\n`,
      [join(cwd, "src", "lib.rs")]: `pub fn ready() {}\n`,
      [rustRegistry(cwd)]: "pub struct Repositories;\n",
    });
    await wireRegister(cwd, "generated", "rust", false, { fs, log: () => undefined });
    expect(files[hookPath(cwd, "rust", true)]).toContain("#[path");
    expect(files[join(cwd, "src", "lib.rs")]).toContain("mod datasources_register;");
    expect(files[join(cwd, "src", "lib.rs")]).toContain("use datasources_register::Repositories;");
    expect(files[join(cwd, "Cargo.toml")]).toContain('ctor = "0.4"');
    await wireRegister(cwd, "generated", "rust", false, { fs, log: () => undefined });
    expect(files[join(cwd, "Cargo.toml")]?.match(/ctor = /g)?.length).toBe(1);
  });

  it("logs a csharp compile include on dry-run", async () => {
    const cwd = "app";
    const { files, fs } = mem(["App.csproj"], {
      [join(cwd, "App.csproj")]: `<Project Sdk="Microsoft.NET.Sdk">\n</Project>\n`,
    });
    const logs: string[] = [];
    await wireRegister(cwd, "/tmp/out", "csharp", true, { fs, log: (line) => logs.push(line) });
    expect(files[join(cwd, "App.csproj")]).toBe(`<Project Sdk="Microsoft.NET.Sdk">\n</Project>\n`);
    expect(logs).toContain(join(cwd, "App.csproj"));
  });

  it("skips a csharp compile include that is already present", async () => {
    const cwd = "app";
    const { files, fs } = mem(["App.csproj"], {
      [join(cwd, "App.csproj")]: `<Project Sdk="Microsoft.NET.Sdk">\n</Project>\n`,
    });
    await wireRegister(cwd, "/tmp/out", "csharp", false, { fs, log: () => undefined });
    const once = files[join(cwd, "App.csproj")];
    await wireRegister(cwd, "/tmp/out", "csharp", false, { fs, log: () => undefined });
    expect(files[join(cwd, "App.csproj")]).toBe(once);
  });

  it("skips csharp when the csproj file cannot be read", async () => {
    const cwd = "app";
    const { files, fs } = mem(["App.csproj"], {});
    await wireRegister(cwd, "/tmp/out", "csharp", false, { fs, log: () => undefined });
    expect(files[join(cwd, "App.csproj")]).toBeUndefined();
  });

  it("skips csharp when the project has no csproj", async () => {
    const cwd = "app";
    const { files, fs } = mem(["App.sln"], {});
    await wireRegister(cwd, "/tmp/out", "csharp", false, { fs, log: () => undefined });
    expect(Object.keys(files)).toHaveLength(0);
  });

  it("adds a csharp compile include when generated output is outside the project", async () => {
    const cwd = "app";
    const { files, fs } = mem(["App.csproj"], {
      [join(cwd, "App.csproj")]: `<Project Sdk="Microsoft.NET.Sdk">\n</Project>\n`,
    });
    await wireRegister(cwd, "/tmp/out", "csharp", false, { fs, log: () => undefined });
    expect(files[join(cwd, "App.csproj")]).toContain("Compile Include=");
    expect(files[join(cwd, "App.csproj")]).toContain("**/*.cs");
  });

  it("skips csharp compile include when generated output is in the project", async () => {
    const cwd = "app";
    const { files, fs } = mem(["App.csproj"], {
      [join(cwd, "App.csproj")]: `<Project Sdk="Microsoft.NET.Sdk">\n</Project>\n`,
    });
    await wireRegister(cwd, "generated", "csharp", false, { fs, log: () => undefined });
    expect(files[join(cwd, "App.csproj")]).toBe(`<Project Sdk="Microsoft.NET.Sdk">\n</Project>\n`);
  });

  it("skips a hook when output is outside the project", async () => {
    const cwd = "app";
    const { files, fs } = mem(["src"], {
      [join(cwd, "src", "index.ts")]: `export const app = 1;\n`,
    });
    await wireRegister(cwd, "/tmp/out", "typescript", false, { fs, log: () => undefined });
    expect(files[hookPath(cwd, "typescript", true)]).toBeUndefined();
    expect(files[join(cwd, "src", "index.ts")]).toBe(`export const app = 1;\n`);
  });

  it("follows folder and file casing for the typescript registry", async () => {
    const cwd = "app";
    const settings = {
      "languages.typescript.casing.directories": "Pascal",
      "languages.typescript.casing.file_names": "kebab",
    };
    const registry = join(cwd, "generated", "typescript/Repositories/repository-registry.ts");
    const { files, fs } = mem(["package.json", "src"], {
      [join(cwd, "src", "index.ts")]: `export const app = 1;\n`,
      [registry]: "export const register = () => {};\n",
    });
    await wireRegister(cwd, "generated", "typescript", false, { fs, log: () => undefined }, settings);
    expect(files[hookPath(cwd, "typescript", true)]).toContain("Repositories/repository-registry.ts");
  });

  it("is idempotent", async () => {
    const cwd = "app";
    const { files, fs } = mem(["src"], {
      [join(cwd, "src", "main.ts")]: `import "./datasources-register.ts";\n`,
      [tsRegistry(cwd)]: "export const register = () => {};\n",
    });
    await wireRegister(cwd, "generated", "typescript", false, { fs, log: () => undefined });
    await wireRegister(cwd, "generated", "typescript", false, { fs, log: () => undefined });
    expect(files[join(cwd, "src", "main.ts")]).toBe(`import "./datasources-register.ts";\n`);
  });
});
