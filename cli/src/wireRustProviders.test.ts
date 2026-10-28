import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Fs } from "./io.ts";
import { attachedProviders, rustBindContents, wireRustProviders } from "./wireRustProviders.ts";

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
      readdir: async () => Object.keys(store),
    },
  };
};

const sqliteRegister = `pub fn register() {
    datasources::register("sqlite", std::sync::Arc::new(datasources_sqlite::SqliteDriver));
}
`;

describe("wireRustProviders", () => {
  it("names each cargo crate", () => {
    expect(rustBindContents(["sqlite", "postgres"])).toBe(
      `pub fn register() {
    datasources::register("sqlite", std::sync::Arc::new(datasources_sqlite::SqliteDriver));
    datasources::register("postgres", std::sync::Arc::new(datasources_postgres::PostgresDriver));
}
`,
    );
  });

  it("skips attach when the bind module and register call are already present", () => {
    expect(
      attachedProviders("mod datasources_providers;\nfn main() {\n    datasources_providers::register();\n}\n"),
    ).toBeUndefined();
  });

  it("inserts register at the start of main", () => {
    expect(attachedProviders("fn main() {}\n")).toBe(
      "mod datasources_providers;\nfn main() {\n    datasources_providers::register();\n}\n",
    );
    expect(attachedProviders("#[tokio::main]\nasync fn main() -> Result<()> {\n    let ds = default()?;\n}\n")).toBe(
      "mod datasources_providers;\n#[tokio::main]\nasync fn main() -> Result<()> {\n    datasources_providers::register();\n    let ds = default()?;\n}\n",
    );
  });

  it("writes the bind file next to src/main.rs", async () => {
    const cwd = "app";
    const { files, fs } = mem({ [join(cwd, "src/main.rs")]: "fn main() {}\n" });
    const logs: string[] = [];
    await wireRustProviders(cwd, ["sqlite"], false, { fs, log: (line) => logs.push(line) });
    expect(files[join(cwd, "src/datasources_providers.rs")]).toBe(sqliteRegister);
    expect(files[join(cwd, "src/main.rs")]).toBe(
      "mod datasources_providers;\nfn main() {\n    datasources_providers::register();\n}\n",
    );
    expect(logs).toContain(join(cwd, "src/datasources_providers.rs"));
    expect(logs).toContain(join(cwd, "src/main.rs"));
  });

  it("logs the bind path on dry-run without writing", async () => {
    const cwd = "app";
    const { files, fs } = mem({ [join(cwd, "src/lib.rs")]: "" });
    const logs: string[] = [];
    await wireRustProviders(cwd, ["json"], true, { fs, log: (line) => logs.push(line) });
    expect(files[join(cwd, "src/datasources_providers.rs")]).toBeUndefined();
    expect(logs).toEqual([join(cwd, "src/datasources_providers.rs")]);
  });

  it("rewrites the bind file when the module is already attached", async () => {
    const cwd = "app";
    const main = "mod datasources_providers;\nfn main() {\n    datasources_providers::register();\n}\n";
    const { files, fs } = mem({ [join(cwd, "src/main.rs")]: main });
    await wireRustProviders(cwd, ["sqlite"], false, { fs, log: () => {} });
    expect(files[join(cwd, "src/datasources_providers.rs")]).toBe(sqliteRegister);
    expect(files[join(cwd, "src/main.rs")]).toBe(main);
  });

  it("writes the bind file at cwd when there is no rust entry", async () => {
    const cwd = "app";
    const { files, fs } = mem();
    const logs: string[] = [];
    await wireRustProviders(cwd, ["json"], false, { fs, log: (line) => logs.push(line) });
    expect(files[join(cwd, "datasources_providers.rs")]).toBe(
      `pub fn register() {
    datasources::register("json", std::sync::Arc::new(datasources_json::JsonDriver));
}
`,
    );
    expect(logs).toEqual([join(cwd, "datasources_providers.rs")]);
  });
});
