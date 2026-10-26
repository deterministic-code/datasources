import { host, type Ecosystem } from "./catalog.ts";

export const detect = (entries: string[]): Ecosystem[] => [
  ...(entries.includes("package.json") ? (["ts"] as const) : []),
  ...(entries.includes("Cargo.toml") ? (["rust"] as const) : []),
  ...(entries.some((e) => e.endsWith(".csproj") || e.endsWith(".sln") || e.endsWith(".slnx"))
    ? (["csharp"] as const)
    : []),
];

export const resolveLanguages = (found: Ecosystem[], explicit: Ecosystem[] = []): Ecosystem[] =>
  explicit.length
    ? explicit
    : found.length
      ? found
      : (() => {
          throw new Error("No package.json, Cargo.toml, or .csproj/.sln in this directory");
        })();

export const hasProject = (language: Ecosystem, entries: string[]): boolean =>
  ({
    csharp: csharpTarget(entries) !== ".",
    rust: entries.includes("Cargo.toml"),
    ts: entries.includes("package.json"),
  })[language];

export const missingProject = (language: Ecosystem): string =>
  ({
    csharp: `No dotnet packages installed for language csharp. Create a project, or run dotnet add package ${host.csharp.core}.`,
    rust: `No cargo packages installed for language rust. Create a project, or run cargo add ${host.rust.core}.`,
    ts: `No npm packages installed for language typescript. Create a project, or run npm install ${host.ts.core}.`,
  })[language];

export const requireProjects = (languages: Ecosystem[], entries: string[]): void => {
  const missing = languages.filter((language) => !hasProject(language, entries));
  if (missing.length) throw new Error(missing.map(missingProject).join("\n"));
};

export const csharpTarget = (entries: string[]): string => {
  const projects = entries.filter((e) => e.endsWith(".csproj")).sort();
  return (
    projects.find((p) => !/tests?\.csproj$/i.test(p)) ??
    projects[0] ??
    entries.find((e) => e.endsWith(".sln") || e.endsWith(".slnx")) ??
    "."
  );
};

export const isEcosystem = (value: string): value is Ecosystem =>
  value === "ts" || value === "rust" || value === "csharp";
