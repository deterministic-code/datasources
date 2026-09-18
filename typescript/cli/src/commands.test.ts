import { describe, expect, it } from "vitest";
import { addCommands, removeCommands } from "./commands.ts";

describe("commands", () => {
  it("adds sqlite with and without an engine on npm", () => {
    expect(addCommands("ts", ["sqlite"], true, ["package.json"])).toEqual([
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
    expect(addCommands("ts", ["text"], false, ["package.json"])[0]?.args).toEqual([
      "install",
      "@datasources/core",
      "@datasources/register",
      "@datasources/text",
    ]);
  });

  it("adds cargo crates and register features", () => {
    expect(addCommands("rust", ["sqlite"], true, ["Cargo.toml"])).toEqual([
      { command: "cargo", args: ["add", "datasources"] },
      { command: "cargo", args: ["add", "datasources-register", "--features", "sqlite"] },
      { command: "cargo", args: ["add", "datasources-sqlite"] },
    ]);
    expect(addCommands("rust", ["sqlite"], false, ["Cargo.toml"])[2]?.args).toEqual([
      "add",
      "datasources-sqlite",
    ]);
    expect(addCommands("rust", ["json"], false, ["Cargo.toml"])[2]?.args).toEqual([
      "add",
      "datasources-json",
    ]);
  });

  it("adds nuget packages on a csproj", () => {
    const cmds = addCommands("csharp", ["sqlite"], true, ["App.csproj"]);
    expect(cmds.map((c) => c.args)).toEqual([
      ["add", "App.csproj", "package", "Datasources"],
      ["add", "App.csproj", "package", "Datasources.Hosting"],
      ["add", "App.csproj", "package", "Datasources.Sqlite"],
      ["add", "App.csproj", "package", "Datasources.Sqlite.Microsoft"],
    ]);
    expect(addCommands("csharp", ["json"], false, [])[0]?.args).toEqual([
      "add",
      "package",
      "Datasources",
    ]);
  });

  it("uninstalls unused engines and dialects", () => {
    expect(removeCommands("ts", ["sqlite"], true, ["json"], [])).toEqual([
      { command: "npm", args: ["uninstall", "@datasources/sqlite"] },
    ]);
    expect(removeCommands("ts", [], true, ["sqlite"], [])).toEqual([]);
    expect(removeCommands("rust", ["sqlite"], true, ["json"], [])).toEqual([
      { command: "cargo", args: ["remove", "datasources-sqlite"] },
      { command: "cargo", args: ["add", "datasources-register", "--features", "json"] },
    ]);
    expect(removeCommands("rust", ["json"], true, [], [])).toEqual([
      { command: "cargo", args: ["remove", "datasources-json"] },
    ]);
    expect(
      removeCommands("csharp", ["sqlite"], true, [], ["App.csproj"]).map((c) => c.args),
    ).toEqual([
      ["remove", "App.csproj", "package", "Datasources.Sqlite"],
      ["remove", "App.csproj", "package", "Datasources.Sqlite.Microsoft"],
    ]);
    expect(removeCommands("csharp", ["json"], false, [], []).map((c) => c.args)).toEqual([
      ["remove", "package", "Datasources.Json"],
    ]);
  });
});
