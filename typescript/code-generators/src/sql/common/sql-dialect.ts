import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import {
  DriverLoader,
  ProviderCatalog,
  ProviderNotInstalledError,
  type IDriver,
  type IStatement,
  type StatementNaming,
} from "@deterministic-code/datasources";

export type SqlDialect = string;

const loaded = new Map<string, IDriver>();

const named = (dialect: string): string =>
  ProviderCatalog.default[dialect] === undefined
    ? (() => {
        throw new Error(`Unknown SQL dialect "${dialect}".`);
      })()
    : dialect;

export const register = (dialect: string, driver: IDriver): void => {
  if (driver.statementRenderer == null) throw new Error(`"${dialect}" is not a SQL dialect.`);
  loaded.set(named(dialect), driver);
};

export const forgetDialects = (): void => {
  loaded.clear();
};

export const dialectDriver = (dialect: string): IDriver => {
  const key = named(dialect);
  const hit = loaded.get(key);
  if (hit !== undefined) return hit;
  const catalog = ProviderCatalog.default[key];
  const driver = DriverLoader.load(catalog);
  if (driver.statementRenderer == null) throw new Error(`"${dialect}" is not a SQL dialect.`);
  loaded.set(key, driver);
  return driver;
};

export const requireDialect = (language: string): SqlDialect => {
  dialectDriver(language);
  return language;
};

const entryOf = (manifest: {
  main?: string;
  module?: string;
  exports?: { "."?: string | { import?: string; default?: string; require?: string } };
}): string => {
  const exp = manifest.exports?.["."];
  return typeof exp === "string"
    ? exp
    : (exp?.import ?? exp?.default ?? exp?.require ?? manifest.main ?? manifest.module ?? "index.js");
};

const consumerPackage = (location: string): string =>
  join(process.cwd(), "node_modules", location, "package.json");

export const ensureDialect = async (language: string): Promise<SqlDialect> => {
  const dialect = named(language);
  if (loaded.has(dialect)) return dialect;
  try {
    dialectDriver(dialect);
    return dialect;
  } catch {
    const catalog = ProviderCatalog.default[dialect];
    const pkg = consumerPackage(catalog.location);
    if (!existsSync(pkg)) throw new ProviderNotInstalledError(dialect);
    const entry = entryOf(JSON.parse(readFileSync(pkg, "utf8")) as Parameters<typeof entryOf>[0]);
    const loadedModule = (await import(pathToFileURL(join(process.cwd(), "node_modules", catalog.location, entry)).href)) as Record<
      string,
      new () => IDriver
    >;
    const Type = loadedModule[catalog.module];
    if (typeof Type !== "function") throw new ProviderNotInstalledError(dialect);
    register(dialect, new Type());
    return dialect;
  }
};

export const schemaRenderer = (dialect: string, naming: StatementNaming) =>
  dialectDriver(dialect).statementRenderer!.withNaming(naming);

export const renderSql = (dialect: string, statement: IStatement): string => {
  const sql = dialectDriver(dialect).statementRenderer!.render(statement).trimEnd();
  return sql === "" ? "" : /[;/]$/.test(sql) ? sql : `${sql};`;
};

export const dialectsFromSettings = (
  raw: string | undefined,
): SqlDialect[] => {
  if (raw === undefined || raw === "") return ["sqlite"];
  const out: SqlDialect[] = [];
  for (const name of raw.split(",")) {
    const key = name.trim();
    if (key === "") continue;
    const dialect = requireDialect(key);
    if (!out.includes(dialect)) out.push(dialect);
  }
  return out.length > 0 ? out : ["sqlite"];
};
