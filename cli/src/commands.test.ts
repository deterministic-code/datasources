import { join, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { addCommands, removeCommands } from "./commands.ts";
import { cliDatasourcesRepo } from "./localRepo.ts";

describe("commands", () => {
  it("adds sqlite on npm", () => {
    expect(addCommands("ts", ["sqlite"], ["package.json"])).toEqual([
      {
        command: "npm",
        args: [
          "install",
          "@deterministic-code/datasources",
          "@deterministic-code/datasources-sqlite",
        ],
      },
    ]);
    expect(addCommands("ts", ["xml"], ["package.json"])[0]?.args).toEqual([
      "install",
      "@deterministic-code/datasources",
      "@deterministic-code/datasources-xml",
    ]);
  });

  it("adds cargo crates", () => {
    expect(addCommands("rust", ["sqlite"], ["Cargo.toml"])).toEqual([
      { command: "cargo", args: ["add", "datasources"] },
      { command: "cargo", args: ["add", "tokio", "--features", "macros,rt-multi-thread"] },
      { command: "cargo", args: ["add", "datasources-sqlite"] },
      {
        command: "npm",
        args: [
          "install",
          "@deterministic-code/datasources",
          "@deterministic-code/datasources-sqlite",
        ],
      },
    ]);
    expect(addCommands("rust", ["json"], ["Cargo.toml"])[2]?.args).toEqual([
      "add",
      "datasources-json",
    ]);
    expect(addCommands("rust", ["json"], ["Cargo.toml"]).some((c) => c.command === "npm")).toBe(false);
  });

  it("adds nuget packages on a csproj", () => {
    const cmds = addCommands("csharp", ["sqlite"], ["App.csproj"]);
    expect(cmds.map((c) => c.args)).toEqual([
      ["add", "App.csproj", "package", "Datasources"],
      ["add", "App.csproj", "package", "Datasources.Sqlite"],
      [
        "install",
        "@deterministic-code/datasources",
        "@deterministic-code/datasources-sqlite",
      ],
    ]);
    expect(addCommands("csharp", ["json"], [])[0]?.args).toEqual([
      "add",
      "package",
      "Datasources",
    ]);
    expect(addCommands("csharp", ["json"], []).some((c) => c.command === "npm")).toBe(false);
    expect(
      addCommands(
        "csharp",
        ["sqlite"],
        ["App.csproj"],
        `<ProjectReference Include="..\\Datasources.csproj" />\n<ProjectReference Include="Datasources.Sqlite.csproj" />`,
      ).map((c) => c.args),
    ).toEqual([
      [
        "install",
        "@deterministic-code/datasources",
        "@deterministic-code/datasources-sqlite",
      ],
    ]);
  });

  it("uninstalls unused dialects", () => {
    expect(removeCommands("ts", ["sqlite"], ["json"], [])).toEqual([
      { command: "npm", args: ["uninstall", "@deterministic-code/datasources-sqlite"] },
    ]);
    expect(removeCommands("ts", [], ["sqlite"], [])).toEqual([]);
    expect(removeCommands("rust", ["sqlite"], ["json"], [])).toEqual([
      { command: "cargo", args: ["remove", "datasources-sqlite"] },
      { command: "npm", args: ["uninstall", "@deterministic-code/datasources-sqlite"] },
    ]);
    expect(removeCommands("rust", ["json"], [], [])).toEqual([
      { command: "cargo", args: ["remove", "datasources-json"] },
    ]);
    expect(removeCommands("csharp", ["sqlite"], [], ["App.csproj"]).map((c) => c.args)).toEqual([
      ["remove", "App.csproj", "package", "Datasources.Sqlite"],
      ["uninstall", "@deterministic-code/datasources-sqlite"],
    ]);
    expect(removeCommands("csharp", ["json"], [], []).map((c) => c.args)).toEqual([
      ["remove", "package", "Datasources.Json"],
    ]);
  });

  it("removes a csharp project reference added with --local", () => {
    const local = { repo: cliDatasourcesRepo, cwd: "app" };
    const include = `..\\..\\..\\..\\opt\\repo\\csharp\\Providers\\Sqlite\\Datasources.Sqlite.csproj`;
    expect(
      removeCommands("csharp", ["sqlite"], [], ["App.csproj"], `<ProjectReference Include="${include}" />`, local).map(
        (c) => c.args,
      ),
    ).toEqual([
      ["remove", "App.csproj", "reference", include],
      ["uninstall", "@deterministic-code/datasources-sqlite"],
    ]);
    expect(
      removeCommands(
        "csharp",
        ["sqlite"],
        [],
        ["App.csproj"],
        `<PackageReference Include="Datasources.Sqlite" />`,
        local,
      ).map((c) => c.args),
    ).toEqual([
      ["remove", "App.csproj", "package", "Datasources.Sqlite"],
      ["uninstall", "@deterministic-code/datasources-sqlite"],
    ]);
    expect(removeCommands("csharp", ["sqlite"], [], ["App.csproj"], undefined, local).map((c) => c.args)).toEqual([
      ["uninstall", "@deterministic-code/datasources-sqlite"],
    ]);
  });

  it("adds from this checkout with --local", () => {
    const root = cliDatasourcesRepo;
    const local = { repo: root, cwd: "app" };
    const builds = [
      ["run", "build", "--prefix", resolve(root, "typescript/datasources")],
      ["run", "build", "--prefix", resolve(root, "typescript/code-generators")],
      ["run", "build", "--prefix", resolve(root, "typescript/providers/sqlite")],
    ];
    const npmLocal = [
      "install",
      "--install-links",
      `file:${resolve(root, "typescript/datasources")}`,
      `file:${resolve(root, "typescript/code-generators")}`,
      `file:${resolve(root, "typescript/providers/sqlite")}`,
      "@deterministic-code/generators-common",
      "@deterministic-code/deterministic-generators-common",
    ];
    expect(addCommands("ts", ["sqlite"], ["package.json"], undefined, local).map((c) => c.args)).toEqual([
      ...builds,
      npmLocal,
    ]);
    expect(addCommands("rust", ["sqlite"], ["Cargo.toml"], undefined, local).map((c) => c.args)).toEqual([
      ["add", "--path", resolve(root, "rust/datasources")],
      ["add", "tokio", "--features", "macros,rt-multi-thread"],
      ["add", "--path", resolve(root, "rust/providers/sqlite")],
      ["add", "--path", resolve(root, "rust/datasources-tests")],
      ...builds,
      npmLocal,
    ]);
    expect(addCommands("csharp", ["sqlite"], ["App.csproj"], undefined, local).map((c) => c.args)).toEqual([
      ["add", "App.csproj", "reference", resolve(root, "csharp/Datasources/Datasources.csproj")],
      ["add", "App.csproj", "reference", resolve(root, "csharp/Providers/Sqlite/Datasources.Sqlite.csproj")],
      ...builds,
      npmLocal,
    ]);
  });

  it("resolves --local file specs to absolute paths", () => {
    const specs = addCommands("ts", ["sqlite"], ["package.json"], undefined, { repo: ".", cwd: "app" })
      .at(-1)!
      .args.filter((arg) => arg.startsWith("file:"));
    expect(specs).toEqual([
      `file:${resolve(".", "typescript/datasources")}`,
      `file:${resolve(".", "typescript/code-generators")}`,
      `file:${resolve(".", "typescript/providers/sqlite")}`,
    ]);
  });
});
