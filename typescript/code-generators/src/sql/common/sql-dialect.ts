import {
  DriverLoader,
  ProviderCatalog,
  type DefaultValueConverter,
  type DeterministicTypeConverter,
  type IDriver,
  type IStatement,
  type SpecField,
  type StatementTextRenderer,
} from "@deterministic-code/datasources";

export type SqlDialect = string;

export type SqlGenerateDriver = IDriver & {
  quoteLeft: string;
  quoteRight: string;
  typeConverter: DeterministicTypeConverter;
  defaultValueConverter: DefaultValueConverter;
  statementRenderer: StatementTextRenderer;
};

const loaded = new Map<string, SqlGenerateDriver>();

const named = (dialect: string): string =>
  ProviderCatalog.default[dialect] === undefined
    ? (() => {
        throw new Error(`Unknown SQL dialect "${dialect}".`);
      })()
    : dialect;

const asGenerate = (dialect: string, driver: IDriver): SqlGenerateDriver =>
  driver.typeConverter == null ||
  driver.defaultValueConverter == null ||
  driver.statementRenderer == null
    ? (() => {
        throw new Error(`"${dialect}" is not a SQL dialect.`);
      })()
    : (driver as SqlGenerateDriver);

export const register = (dialect: string, driver: IDriver): void => {
  loaded.set(named(dialect), asGenerate(dialect, driver));
};

export const dialectDriver = (dialect: string): SqlGenerateDriver => {
  const key = named(dialect);
  const hit = loaded.get(key);
  if (hit !== undefined) return hit;
  const catalog = ProviderCatalog.default[key];
  const driver = asGenerate(key, DriverLoader.load(catalog));
  loaded.set(key, driver);
  return driver;
};

export const renderSql = (dialect: string, statement: IStatement): string => {
  const sql = dialectDriver(dialect).statementRenderer.render(statement).trimEnd();
  return sql === "" ? "" : /[;/]$/.test(sql) ? sql : `${sql};`;
};

export const dialectConverter = (dialect: string): {
  quoteLeft: string;
  quoteRight: string;
  defaultValueConverter: DefaultValueConverter;
  toColumnType: (field: SpecField) => string;
} => {
  const driver = dialectDriver(dialect);
  return {
    quoteLeft: driver.quoteLeft,
    quoteRight: driver.quoteRight,
    defaultValueConverter: driver.defaultValueConverter,
    toColumnType: (field) => driver.typeConverter.convert(field),
  };
};

export const requireDialect = (language: string): SqlDialect => {
  dialectDriver(language);
  return language;
};

/** Dialects listed in `backend.datasources`. */
export const dialectsFromSettings = (
  settings: Record<string, string>,
): SqlDialect[] => {
  const raw = settings["backend.datasources"];
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

export const q = (dialect: string, ident: string): string => {
  const driver = dialectDriver(dialect);
  return `${driver.quoteLeft}${ident}${driver.quoteRight}`;
};

export const mapColumnType = (dialect: string, field: SpecField): string =>
  dialectDriver(dialect).typeConverter.convert(field);

export const sqlDefault = (dialect: string, field: SpecField): string | null => {
  if (field.defaultValue === undefined || field.defaultValue === null) return null;
  return dialectDriver(dialect).defaultValueConverter.convert(
    field.type,
    String(field.defaultValue),
  );
};
