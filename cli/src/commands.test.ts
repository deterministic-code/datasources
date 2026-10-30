import { describe, expect, it } from "vitest";
import { addCommands, removeCommands } from "./commands.ts";

describe("commands", () => {
  it("adds sqlite on npm", () => {
    expect(addCommands("ts", ["sqlite"], ["package.json"])).toEqual([
      {
        command: "npm",
        args: ["install", "@deterministic-code/datasources", "@deterministic-code/datasources-sqlite"],
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
      { command: "cargo", args: ["add", "ctor@0.4"] },
      { command: "cargo", args: ["add", "datasources-sqlite"] },
      {
        command: "npm",
        args: ["install", "@deterministic-code/datasources", "@deterministic-code/datasources-sqlite"],
      },
    ]);
    expect(addCommands("rust", ["json"], ["Cargo.toml"])[3]?.args).toEqual([
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
      ["install", "@deterministic-code/datasources", "@deterministic-code/datasources-sqlite"],
    ]);
    expect(addCommands("csharp", ["json"], [])[0]?.args).toEqual([
      "add",
      "package",
      "Datasources",
    ]);
    expect(addCommands("csharp", ["json"], []).some((c) => c.command === "npm")).toBe(false);
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
});
