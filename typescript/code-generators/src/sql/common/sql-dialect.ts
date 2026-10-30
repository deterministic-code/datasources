import {
  DriverLoader,
  ProviderCatalog,
  type IDriver,
  type IStatement,
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
