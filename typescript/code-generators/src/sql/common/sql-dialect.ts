import type {
  DefaultValueConverter,
  DeterministicTypeConverter,
  IDriver,
  SpecField,
} from "@datasources/core";
import { MysqlDriver } from "@datasources/mysql";
import { OracleDriver } from "@datasources/oracle";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";
import { SqlServerDriver } from "@datasources/sqlserver";

const SQL_DIALECTS = [
  "sqlite",
  "mysql",
  "postgres",
  "sqlserver",
  "oracle",
] as const;

export type SqlDialect = (typeof SQL_DIALECTS)[number];

export type SqlGenerateDriver = IDriver & {
  quoteLeft: string;
  quoteRight: string;
  typeConverter: DeterministicTypeConverter;
  defaultValueConverter: DefaultValueConverter;
};

const drivers: Record<SqlDialect, () => SqlGenerateDriver> = {
  sqlite: () => new SqliteDriver(),
  mysql: () => new MysqlDriver(),
  postgres: () => new PostgresDriver(),
  sqlserver: () => new SqlServerDriver(),
  oracle: () => new OracleDriver(),
};

export const dialectDriver = (dialect: string): SqlGenerateDriver => {
  const create = drivers[dialect as SqlDialect];
  if (create === undefined) {
    throw new Error(
      `Unknown SQL dialect "${dialect}". Valid: ${SQL_DIALECTS.join(", ")}.`,
    );
  }
  return create();
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
  return language as SqlDialect;
};

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
