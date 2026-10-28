import { dirname, join } from "node:path";
import { catalog, type Provider } from "./catalog.ts";
import type { Fs } from "./io.ts";

const rustDriver: Record<Provider, string> = {
  json: "JsonDriver",
  memory: "MemoryDriver",
  sqlite: "SqliteDriver",
  xml: "XmlDriver",
  yaml: "YamlDriver",
  mysql: "MysqlDriver",
  sqlserver: "SqlServerDriver",
  postgres: "PostgresDriver",
  oracle: "OracleDriver",
  mongodb: "MongoDbDriver",
  dynamodb: "DynamoDbDriver",
};

const optional = async (fs: Fs, path: string): Promise<string | undefined> => {
  try {
    return await fs.readFile(path);
  } catch {
    return undefined;
  }
};

const firstFile = async (fs: Fs, cwd: string, names: string[]): Promise<{ path: string; text: string } | undefined> => {
  for (const name of names) {
    const path = join(cwd, name);
    const text = await optional(fs, path);
    if (text !== undefined) return { path, text };
  }
  return undefined;
};

export const rustBindContents = (names: Provider[]): string =>
  `pub fn register() {\n${names
    .map((name) => {
      const crate = catalog[name].cargo.replaceAll("-", "_");
      return `    datasources::register("${name}", std::sync::Arc::new(${crate}::${rustDriver[name]}));`;
    })
    .join("\n")}\n}\n`;

const withMod = (entry: string): string =>
  entry.includes("datasources_providers") ? entry : `mod datasources_providers;\n${entry}`;

const withRegisterCall = (entry: string): string => {
  if (entry.includes("datasources_providers::register()")) return entry;
  const main = /((?:async\s+)?fn\s+main\s*\([^)]*\)(?:\s*->\s*[^{]+)?\s*)\{(\s*)/;
  return entry.replace(main, (_, sig: string, ws: string) => `${sig}{\n    datasources_providers::register();${ws === "" ? "\n" : ws}`);
};

export const attachedProviders = (entry: string): string | undefined => {
  const next = withRegisterCall(withMod(entry));
  return next === entry ? undefined : next;
};

export const wireRustProviders = async (
  cwd: string,
  names: Provider[],
  dryRun: boolean,
  deps: { fs: Fs; log: (line: string) => void },
): Promise<void> => {
  const entry = await firstFile(deps.fs, cwd, ["src/lib.rs", "src/main.rs", "lib.rs", "main.rs"]);
  const bind = join(entry ? dirname(entry.path) : cwd, "datasources_providers.rs");
  const text = rustBindContents(names);
  if (dryRun) {
    deps.log(bind);
    return;
  }
  await deps.fs.writeFile(bind, text);
  deps.log(bind);
  if (!entry) return;
  const next = attachedProviders(entry.text);
  if (next === undefined) return;
  await deps.fs.writeFile(entry.path, next);
  deps.log(entry.path);
};
