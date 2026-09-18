import { catalog, host, type Ecosystem, type Provider } from "./catalog.ts";
import { csharpTarget } from "./detect.ts";

export type Proc = { command: string; args: string[] };

export const addCommands = (ecosystem: Ecosystem, names: Provider[], entries: string[]): Proc[] =>
  ({
    ts: () => npmInstall(names),
    rust: () => cargoAdd(names),
    csharp: () => dotnetAdd(names, csharpTarget(entries)),
  })[ecosystem]();

export const removeCommands = (
  ecosystem: Ecosystem,
  names: Provider[],
  remaining: Provider[],
  entries: string[],
): Proc[] =>
  ({
    ts: () => npmUninstall(names),
    rust: () => cargoRemove(names, remaining),
    csharp: () => dotnetRemove(names, csharpTarget(entries)),
  })[ecosystem]();

const npmInstall = (names: Provider[]): Proc[] => [
  {
    command: "npm",
    args: [
      "install",
      host.ts.core,
      host.ts.register,
      ...names.map((name) => catalog[name].npm),
    ],
  },
];

const npmUninstall = (names: Provider[]): Proc[] => {
  const pkgs = names.map((name) => catalog[name].npm);
  return pkgs.length ? [{ command: "npm", args: ["uninstall", ...pkgs] }] : [];
};

const cargoAdd = (names: Provider[]): Proc[] => [
  { command: "cargo", args: ["add", host.rust.core] },
  {
    command: "cargo",
    args: ["add", host.rust.register, "--features", names.join(",")],
  },
  ...names.map((name) => ({
    command: "cargo",
    args: ["add", catalog[name].cargo],
  })),
];

const cargoRemove = (names: Provider[], remaining: Provider[]): Proc[] => [
  ...names.map((name) => ({ command: "cargo", args: ["remove", catalog[name].cargo] })),
  ...(remaining.length
    ? [{ command: "cargo", args: ["add", host.rust.register, "--features", remaining.join(",")] }]
    : []),
];

const dotnetAdd = (names: Provider[], project: string): Proc[] => {
  const prefix = project === "." ? [] : [project];
  return [
    { command: "dotnet", args: ["add", ...prefix, "package", host.csharp.core] },
    { command: "dotnet", args: ["add", ...prefix, "package", host.csharp.register] },
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
