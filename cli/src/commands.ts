import { catalog, host, type Ecosystem, type Provider } from "./catalog.ts";
import { csharpTarget } from "./detect.ts";
import { sqlMigrateProviders } from "./migrateInit.ts";

export type Proc = { command: string; args: string[] };

const npmDialects = (names: Provider[]): Proc[] => {
  const dialects = sqlMigrateProviders(names);
  return dialects.length === 0 ? [] : npmInstall(dialects);
};

export const addCommands = (ecosystem: Ecosystem, names: Provider[], entries: string[]): Proc[] =>
  ({
    ts: () => npmInstall(names),
    rust: () => [...cargoAdd(names), ...npmDialects(names)],
    csharp: () => [...dotnetAdd(names, csharpTarget(entries)), ...npmDialects(names)],
  })[ecosystem]();

export const removeCommands = (
  ecosystem: Ecosystem,
  names: Provider[],
  _remaining: Provider[],
  entries: string[],
): Proc[] =>
  ({
    ts: () => npmUninstall(names),
    rust: () => [...cargoRemove(names), ...npmUninstall(sqlMigrateProviders(names))],
    csharp: () => [...dotnetRemove(names, csharpTarget(entries)), ...npmUninstall(sqlMigrateProviders(names))],
  })[ecosystem]();

const npmInstall = (names: Provider[]): Proc[] => [
  {
    command: "npm",
    args: ["install", host.ts.core, ...names.map((name) => catalog[name].npm)],
  },
];

const npmUninstall = (names: Provider[]): Proc[] => {
  const pkgs = names.map((name) => catalog[name].npm);
  return pkgs.length ? [{ command: "npm", args: ["uninstall", ...pkgs] }] : [];
};

const cargoAdd = (names: Provider[]): Proc[] => [
  { command: "cargo", args: ["add", host.rust.core] },
  { command: "cargo", args: ["add", "tokio", "--features", "macros,rt-multi-thread"] },
  { command: "cargo", args: ["add", "ctor@0.4"] },
  ...names.map((name) => ({
    command: "cargo",
    args: ["add", catalog[name].cargo],
  })),
];

const cargoRemove = (names: Provider[]): Proc[] =>
  names.map((name) => ({ command: "cargo", args: ["remove", catalog[name].cargo] }));

const dotnetAdd = (names: Provider[], project: string): Proc[] => {
  const prefix = project === "." ? [] : [project];
  return [
    { command: "dotnet", args: ["add", ...prefix, "package", host.csharp.core] },
    ...names.map((name) => ({
      command: "dotnet",
      args: ["add", ...prefix, "package", catalog[name].nuget],
    })),
  ];
};

const dotnetRemove = (names: Provider[], project: string): Proc[] => {
  const prefix = project === "." ? [] : [project];
  return names.map((name) => ({
    command: "dotnet",
    args: ["remove", ...prefix, "package", catalog[name].nuget],
  }));
};
