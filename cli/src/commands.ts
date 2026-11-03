import { resolve } from "node:path";
import { catalog, host, type Ecosystem, type Provider } from "./catalog.ts";
import { csharpTarget } from "./detect.ts";
import { hasCsharpProjectReference } from "./installed.ts";
import { sqlMigrateProviders } from "./migrateInit.ts";

export type Proc = { command: string; args: string[] };

export type LocalInstall = { repo: string; cwd: string };

const npmDialects = (names: Provider[], local?: LocalInstall): Proc[] => {
  const dialects = sqlMigrateProviders(names);
  return dialects.length === 0 ? [] : npmInstall(dialects, local);
};

export const addCommands = (
  ecosystem: Ecosystem,
  names: Provider[],
  entries: string[],
  csproj?: string,
  local?: LocalInstall,
): Proc[] =>
  ({
    ts: () => npmInstall(names, local),
    rust: () => [...cargoAdd(names, local), ...npmDialects(names, local)],
    csharp: () => [...dotnetAdd(names, csharpTarget(entries), csproj, local), ...npmDialects(names, local)],
  })[ecosystem]();

export const removeCommands = (
  ecosystem: Ecosystem,
  names: Provider[],
  _remaining: Provider[],
  entries: string[],
  csproj?: string,
  local?: LocalInstall,
): Proc[] =>
  ({
    ts: () => npmUninstall(names),
    rust: () => [...cargoRemove(names), ...npmUninstall(sqlMigrateProviders(names))],
    csharp: () => [
      ...dotnetRemove(names, csharpTarget(entries), csproj, local),
      ...npmUninstall(sqlMigrateProviders(names)),
    ],
  })[ecosystem]();

const fileSpec = (local: LocalInstall, path: string): string => `file:${resolve(local.repo, path)}`;

const generatorPkgs = [
  "@deterministic-code/generators-common",
  "@deterministic-code/deterministic-generators-common",
];

const localBuild = (local: LocalInstall, names: Provider[]): Proc[] =>
  [
    "typescript/datasources",
    "typescript/code-generators",
    ...names.map((name) => `typescript/providers/${name}`),
  ].map((path) => ({
    command: "npm",
    args: ["run", "build", "--prefix", resolve(local.repo, path)],
  }));

const npmInstall = (names: Provider[], local?: LocalInstall): Proc[] => [
  ...(local ? localBuild(local, names) : []),
  {
    command: "npm",
    args: [
      "install",
      ...(local
        ? [
            "--install-links",
            fileSpec(local, "typescript/datasources"),
            fileSpec(local, "typescript/code-generators"),
            ...names.map((name) => fileSpec(local, `typescript/providers/${name}`)),
            ...generatorPkgs,
          ]
        : [host.ts.core, ...names.map((name) => catalog[name].npm)]),
    ],
  },
];

const npmUninstall = (names: Provider[]): Proc[] => {
  const pkgs = names.map((name) => catalog[name].npm);
  return pkgs.length ? [{ command: "npm", args: ["uninstall", ...pkgs] }] : [];
};

const cargoAdd = (names: Provider[], local?: LocalInstall): Proc[] => [
  {
    command: "cargo",
    args: local ? ["add", "--path", resolve(local.repo, "rust/datasources")] : ["add", host.rust.core],
  },
  { command: "cargo", args: ["add", "tokio", "--features", "macros,rt-multi-thread"] },
  ...names.map((name) => ({
    command: "cargo",
    args: local ? ["add", "--path", resolve(local.repo, `rust/providers/${name}`)] : ["add", catalog[name].cargo],
  })),
  ...(local ? [{ command: "cargo", args: ["add", "--path", resolve(local.repo, "rust/datasources-tests")] }] : []),
];

const cargoRemove = (names: Provider[]): Proc[] =>
  names.map((name) => ({ command: "cargo", args: ["remove", catalog[name].cargo] }));

const csharpCsproj = (local: LocalInstall, nuget: string): string =>
  nuget === host.csharp.core
    ? resolve(local.repo, "csharp/Datasources/Datasources.csproj")
    : resolve(local.repo, `csharp/Providers/${nuget.slice("Datasources.".length)}/${nuget}.csproj`);

const dotnetAdd = (names: Provider[], project: string, csproj?: string, local?: LocalInstall): Proc[] => {
  const prefix = project === "." ? [] : [project];
  const ids = [host.csharp.core, ...names.map((name) => catalog[name].nuget)].filter(
    (id, index, all) => all.indexOf(id) === index && (csproj === undefined || !hasCsharpProjectReference(csproj, id)),
  );
  return ids.map((id) =>
    local
      ? { command: "dotnet", args: ["add", ...prefix, "reference", csharpCsproj(local, id)] }
      : { command: "dotnet", args: ["add", ...prefix, "package", id] },
  );
};

const projectReferenceInclude = (xml: string, id: string): string | undefined => {
  for (const match of xml.matchAll(/Include="([^"]+)"/g)) {
    const include = match[1];
    if (include !== undefined && include.endsWith(`${id}.csproj`)) return include;
  }
};

const hasPackageReference = (xml: string, id: string): boolean => xml.includes(`PackageReference Include="${id}"`);

const dotnetRemove = (names: Provider[], project: string, csproj?: string, local?: LocalInstall): Proc[] => {
  const prefix = project === "." ? [] : [project];
  if (local === undefined) {
    return names.map((name) => ({
      command: "dotnet",
      args: ["remove", ...prefix, "package", catalog[name].nuget],
    }));
  }
  return names.flatMap((name) => {
    const id = catalog[name].nuget;
    const include = csproj === undefined ? undefined : projectReferenceInclude(csproj, id);
    return [
      ...(include === undefined ? [] : [{ command: "dotnet", args: ["remove", ...prefix, "reference", include] }]),
      ...(csproj !== undefined && hasPackageReference(csproj, id)
        ? [{ command: "dotnet", args: ["remove", ...prefix, "package", id] }]
        : []),
    ];
  });
};
