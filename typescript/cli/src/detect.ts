import type { Ecosystem } from "./catalog.ts";

export const detect = (entries: string[]): Ecosystem[] => [
  ...(entries.includes("package.json") ? (["ts"] as const) : []),
  ...(entries.includes("Cargo.toml") ? (["rust"] as const) : []),
  ...(entries.some((e) => e.endsWith(".csproj") || e.endsWith(".sln") || e.endsWith(".slnx"))
    ? (["csharp"] as const)
    : []),
];

export const resolveEcosystem = (found: Ecosystem[], explicit?: Ecosystem): Ecosystem => {
  if (explicit) return explicit;
  if (found.length === 1) return found[0]!;
  throw new Error(
    found.length === 0
      ? "No package.json, Cargo.toml, or .csproj/.sln in this directory"
      : "Multiple project types found. Pass --ecosystem ts|rust|csharp",
  );
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
