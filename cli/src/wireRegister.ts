import { dirname, isAbsolute, join, relative, sep } from "node:path";
import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { csharpTarget } from "./detect.ts";
import { repositoriesPrefix } from "./generate/casing.ts";
import { rustIdent } from "./generate/RepositoryModel.ts";
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

const registryRel = (dialect: "typescript" | "rust", settings: Record<string, string>): string => {
  const casing = createCasingStrategy(dialect, settings);
  const dir = repositoriesPrefix(dialect, casing);
  return dialect === "typescript"
    ? `${dir}/${casing.convertFileName("repository_registry")}.ts`
    : `${dir}/mod.rs`;
};

export const hookContents = (
  dialect: "typescript" | "rust",
  hook: string,
  output: string,
  settings: Record<string, string> = {},
): string => {
  const casing = createCasingStrategy(dialect, settings);
  return dialect === "typescript"
    ? `import "${specFrom(hook, join(output, registryRel(dialect, settings)))}";\n`
    : `#[path = "${specFrom(hook, join(output, dialect, "mod.rs"))}"]\npub mod generated;\npub use generated::*;\npub use generated::repositories::${rustIdent(casing.convertFileName("repository_registry"))}::Repositories;\n`;
};

const rustCargoLine = (cargo: string, name: string, line: string): string | undefined =>
  new RegExp(`^\\s*${name}\\s*=`, "m").test(cargo) ? undefined : line;

export const cargoWithCtor = (cargo: string): string | undefined => {
  const add = [
    rustCargoLine(cargo, "ctor", 'ctor = "0.4"'),
    rustCargoLine(cargo, "tokio", 'tokio = { version = "1", features = ["macros", "rt-multi-thread"] }'),
    rustCargoLine(cargo, "chrono", 'chrono = { version = "0.4", default-features = false, features = ["clock"] }'),
    rustCargoLine(cargo, "uuid", 'uuid = { version = "1", features = ["v4"] }'),
    rustCargoLine(cargo, "rust_decimal", 'rust_decimal = "1"'),
  ].filter((line): line is string => line !== undefined);
  if (add.length === 0) return undefined;
  const block = add.join("\n");
  return cargo.includes("[dependencies]")
    ? cargo.replace("[dependencies]", `[dependencies]\n${block}`)
    : `${cargo.trimEnd()}\n\n[dependencies]\n${block}\n`;
};

export const attached = (dialect: "typescript" | "rust", entry: string, entryPath: string, hook: string): string | undefined => {
  if (dialect === "typescript")
    return entry.includes("datasources-register") ? undefined : `import "${specFrom(entryPath, hook)}";\n${entry}`;
  const register = entry.includes("datasources_register")
    ? entry
    : `mod datasources_register;\nuse datasources_register::Repositories;\n\n${entry}`;
  const next = register.includes("mod types;") ? register : `mod types;\n${register}`;
  return next === entry ? undefined : next;
};

const optional = async (fs: Fs, path: string): Promise<string | undefined> => {
  try {
    return await fs.readFile(path);
  } catch {
    return undefined;
  }
};

const outputRoot = (cwd: string, output: string): string => (isAbsolute(output) ? output : join(cwd, output));

const registryPath = (root: string, dialect: "typescript" | "rust", settings: Record<string, string>): string =>
  join(root, registryRel(dialect, settings));

const firstFile = async (fs: Fs, cwd: string, names: string[]): Promise<{ path: string; text: string } | undefined> => {
  for (const name of names) {
    const path = join(cwd, name);
    const text = await optional(fs, path);
    if (text !== undefined) return { path, text };
  }
  return undefined;
};

const excludeNodeModules =
  "  <PropertyGroup>\n    <DefaultItemExcludes>$(DefaultItemExcludes);node_modules/**</DefaultItemExcludes>\n  </PropertyGroup>\n";

const wireCsharp = async (cwd: string, output: string, dryRun: boolean, deps: Log): Promise<void> => {
  const entries = await deps.fs.readdir(cwd).catch(() => []);
  const target = csharpTarget(entries);
  if (!target.endsWith(".csproj")) return;
  const csprojPath = join(cwd, target);
  const generated = join(outputRoot(cwd, output), "csharp");
  const rel = posix(relative(dirname(csprojPath), generated));
  const include = `${rel}/**/*.cs`;
  const existing = await optional(deps.fs, csprojPath);
  if (existing === undefined) return;
  const needExclude = !existing.includes("node_modules/**");
  const needInclude = rel.startsWith("..") && !existing.includes(include);
  if (!needExclude && !needInclude) return;
  if (dryRun) {
    deps.log(csprojPath);
    return;
  }
  const patched = [
    needExclude ? excludeNodeModules : "",
    needInclude ? `  <ItemGroup>\n    <Compile Include="${include}" />\n  </ItemGroup>\n` : "",
  ].join("");
  await deps.fs.writeFile(csprojPath, existing.replace("</Project>", `${patched}</Project>`));
  deps.log(csprojPath);
};

export const wireRegister = async (
  cwd: string,
  output: string,
  dialect: Dialect,
  dryRun: boolean,
  deps: Log,
  settings: Record<string, string> = {},
): Promise<void> => {
  if (dialect === "csharp") return wireCsharp(cwd, output, dryRun, deps);
  const root = outputRoot(cwd, output);
  if (relative(cwd, root).startsWith("..")) return;
  if ((await optional(deps.fs, registryPath(root, dialect, settings))) === undefined) return;
  const entries = await deps.fs.readdir(cwd).catch(() => []);
  const hook = hookPath(cwd, dialect, entries.includes("src"));
  const text = hookContents(dialect, hook, root, settings);
  if (dryRun) {
    deps.log(hook);
    return;
  }
  await deps.fs.writeFile(hook, text);
  deps.log(hook);
  if (dialect === "rust") {
    const types = join(dirname(hook), "types.rs");
    await deps.fs.writeFile(
      types,
      "pub mod generated {\n    pub use crate::datasources_register::generated as views;\n    pub use crate::datasources_register::generated as datasource;\n}\n",
    );
    deps.log(types);
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
