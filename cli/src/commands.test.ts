import { describe, expect, it } from "vitest";
import { addCommands, removeCommands } from "./commands.ts";

describe("commands", () => {
  it("adds sqlite on npm", () => {
    expect(addCommands("ts", ["sqlite"], ["package.json"])).toEqual([
      {
        command: "npm",
        args: [
          "install",
          "@datasources/core",
          "@datasources/register",
          "@datasources/sqlite",
        ],
      },
    ]);
    expect(addCommands("ts", ["xml"], ["package.json"])[0]?.args).toEqual([
      "install",
      "@datasources/core",
      "@datasources/register",
      "@datasources/xml",
    ]);
  });

  it("adds cargo crates and register features", () => {
    expect(addCommands("rust", ["sqlite"], ["Cargo.toml"])).toEqual([
      { command: "cargo", args: ["add", "datasources"] },
      { command: "cargo", args: ["add", "datasources-register", "--features", "sqlite"] },
      { command: "cargo", args: ["add", "datasources-sqlite"] },
    ]);
    expect(addCommands("rust", ["json"], ["Cargo.toml"])[2]?.args).toEqual([
      "add",
      "datasources-json",
    ]);
  });

  it("adds nuget packages on a csproj", () => {
    const cmds = addCommands("csharp", ["sqlite"], ["App.csproj"]);
    expect(cmds.map((c) => c.args)).toEqual([
      ["add", "App.csproj", "package", "Datasources"],
      ["add", "App.csproj", "package", "Datasources.Sqlite"],
    ]);
    expect(addCommands("csharp", ["json"], [])[0]?.args).toEqual([
      "add",
      "package",
      "Datasources",
    ]);
  });

  it("uninstalls unused dialects", () => {
    expect(removeCommands("ts", ["sqlite"], ["json"], [])).toEqual([
      { command: "npm", args: ["uninstall", "@datasources/sqlite"] },
    ]);
    expect(removeCommands("ts", [], ["sqlite"], [])).toEqual([]);
    expect(removeCommands("rust", ["sqlite"], ["json"], [])).toEqual([
      { command: "cargo", args: ["remove", "datasources-sqlite"] },
      { command: "cargo", args: ["add", "datasources-register", "--features", "json"] },
    ]);
    expect(removeCommands("rust", ["json"], [], [])).toEqual([
      { command: "cargo", args: ["remove", "datasources-json"] },
    ]);
    expect(removeCommands("csharp", ["sqlite"], [], ["App.csproj"]).map((c) => c.args)).toEqual([
      ["remove", "App.csproj", "package", "Datasources.Sqlite"],
    ]);
    expect(removeCommands("csharp", ["json"], [], []).map((c) => c.args)).toEqual([
      ["remove", "package", "Datasources.Json"],
    ]);
  });
});
