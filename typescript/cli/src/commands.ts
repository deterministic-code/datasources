import { catalog, host, type Ecosystem, type Provider } from "./catalog.ts";
import { csharpTarget } from "./detect.ts";

export type Proc = { command: string; args: string[] };

export const addCommands = (
  ecosystem: Ecosystem,
  names: Provider[],
  engine: boolean,
  entries: string[],
): Proc[] =>
  ({
    ts: () => npmInstall(names),
    rust: () => cargoAdd(names),
    csharp: () => dotnetAdd(names, engine, csharpTarget(entries)),
  })[ecosystem]();

export const removeCommands = (
  ecosystem: Ecosystem,
  names: Provider[],
  engine: boolean,
  remaining: Provider[],
  entries: string[],
): Proc[] =>
  ({
    ts: () => npmUninstall(names),
    rust: () => cargoRemove(names, remaining),
    csharp: () => dotnetRemove(names, engine, remaining, csharpTarget(entries)),
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

const dotnetAdd = (names: Provider[], engine: boolean, project: string): Proc[] => {
  const prefix = project === "." ? [] : [project];
  return [
    { command: "dotnet", args: ["add", ...prefix, "package", host.csharp.core] },
    { command: "dotnet", args: ["add", ...prefix, "package", host.csharp.register] },
    ...names.flatMap((name) => [
      { command: "dotnet", args: ["add", ...prefix, "package", catalog[name].nuget] },
      ...(engine && catalog[name].nugetEngine
        ? [{ command: "dotnet", args: ["add", ...prefix, "package", catalog[name].nugetEngine] }]
        : []),
    ]),
  ];
};

const dotnetRemove = (
  names: Provider[],
  engine: boolean,
  remaining: Provider[],
  project: string,
): Proc[] => {
  const prefix = project === "." ? [] : [project];
  const engines = enginePackages(names, engine, "nugetEngine").filter(
    (pkg) => !enginePackages(remaining, true, "nugetEngine").includes(pkg),
  );
  return [
    ...names.map((name) => ({
      command: "dotnet",
      args: ["remove", ...prefix, "package", catalog[name].nuget],
    })),
    ...engines.map((pkg) => ({ command: "dotnet", args: ["remove", ...prefix, "package", pkg] })),
  ];
};

const enginePackages = (
  names: Provider[],
  engine: boolean,
  key: "nugetEngine",
): string[] =>
  engine
    ? [
        ...new Set(
          names.flatMap((name) => {
            const pkg = catalog[name][key];
            return pkg ? [pkg] : [];
          }),
        ),
      ]
    : [];
