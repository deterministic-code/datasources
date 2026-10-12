import { dirname, isAbsolute, join, relative, sep } from "node:path";
import { csharpTarget } from "./detect.ts";
import type { Dialect } from "./generate/GenerateRequest.ts";
import type { Fs } from "./io.ts";

type Log = { fs: Fs; log: (line: string) => void };

const posix = (path: string): string => path.split(sep).join("/");

export const specFrom = (from: string, to: string): string => {
  const rel = posix(relative(dirname(from), to));
  return rel.startsWith(".") ? rel : `./${rel}`;
};

export const hookPath = (cwd: string, dialect: "typescript" | "rust", src: boolean): string =>
  join(cwd, src ? "src" : ".", dialect === "typescript" ? "datasources-register.ts" : "datasources_register.rs");

export const hookContents = (dialect: "typescript" | "rust", hook: string, output: string): string =>
  dialect === "typescript"
    ? `import "${specFrom(hook, join(output, "typescript/repositories/repository-registry.ts"))}";\n`
    : `#[path = "${specFrom(hook, join(output, "rust/repositories/mod.rs"))}"]\npub mod repositories;\npub use repositories::repository_registry::Repositories;\n`;

export const cargoWithCtor = (cargo: string): string | undefined =>
  /^\s*ctor\s*=/m.test(cargo)
    ? undefined
    : cargo.includes("[dependencies]")
      ? cargo.replace("[dependencies]", `[dependencies]\nctor = "0.4"`)
      : `${cargo.trimEnd()}\n\n[dependencies]\nctor = "0.4"\n`;

export const attached = (dialect: "typescript" | "rust", entry: string, entryPath: string, hook: string): string | undefined =>
  dialect === "typescript"
    ? entry.includes("datasources-register")
      ? undefined
      : `import "${specFrom(entryPath, hook)}";\n${entry}`
    : entry.includes("datasources_register")
      ? undefined
      : `mod datasources_register;\nuse datasources_register::Repositories;\n\n${entry}`;

const optional = async (fs: Fs, path: string): Promise<string | undefined> => {
  try {
    return await fs.readFile(path);
  } catch {
    return undefined;
  }
};

const outputRoot = (cwd: string, output: string): string => (isAbsolute(output) ? output : join(cwd, output));

const firstFile = async (fs: Fs, cwd: string, names: string[]): Promise<{ path: string; text: string } | undefined> => {
  for (const name of names) {
    const path = join(cwd, name);
    const text = await optional(fs, path);
    if (text !== undefined) return { path, text };
  }
  return undefined;
};

const wireCsharp = async (cwd: string, output: string, dryRun: boolean, deps: Log): Promise<void> => {
  const entries = await deps.fs.readdir(cwd).catch(() => []);
  const target = csharpTarget(entries);
  if (!target.endsWith(".csproj")) return;
  const csprojPath = join(cwd, target);
  const generated = join(outputRoot(cwd, output), "csharp");
  const rel = posix(relative(dirname(csprojPath), generated));
  if (!rel.startsWith("..")) return;
  const include = `${rel}/**/*.cs`;
  const existing = await optional(deps.fs, csprojPath);
  if (existing === undefined || existing.includes(include)) return;
  if (dryRun) {
    deps.log(csprojPath);
    return;
  }
  await deps.fs.writeFile(csprojPath, existing.replace("</Project>", `  <ItemGroup>\n    <Compile Include="${include}" />\n  </ItemGroup>\n</Project>`));
  deps.log(csprojPath);
};

export const wireRegister = async (
  cwd: string,
  output: string,
  dialect: Dialect,
  dryRun: boolean,
  deps: Log,
): Promise<void> => {
  if (dialect === "csharp") return wireCsharp(cwd, output, dryRun, deps);
  const root = outputRoot(cwd, output);
  if (relative(cwd, root).startsWith("..")) return;
  const entries = await deps.fs.readdir(cwd).catch(() => []);
  const hook = hookPath(cwd, dialect, entries.includes("src"));
  const text = hookContents(dialect, hook, root);
  if (dryRun) {
    deps.log(hook);
    return;
  }
  await deps.fs.writeFile(hook, text);
  deps.log(hook);
  if (dialect === "rust") {
    const cargoPath = join(cwd, "Cargo.toml");
    const cargo = await optional(deps.fs, cargoPath);
    const nextCargo = cargo === undefined ? undefined : cargoWithCtor(cargo);
    if (nextCargo !== undefined) {
      await deps.fs.writeFile(cargoPath, nextCargo);
      deps.log(cargoPath);
    }
  }
  const entry = await firstFile(
    deps.fs,
    cwd,
    dialect === "typescript" ? ["src/index.ts", "src/main.ts", "index.ts", "main.ts"] : ["src/lib.rs", "src/main.rs"],
  );
  if (!entry) return;
  const next = attached(dialect, entry.text, entry.path, hook);
  if (next === undefined) return;
  await deps.fs.writeFile(entry.path, next);
  deps.log(entry.path);
};
