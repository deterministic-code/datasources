import { createRequire } from "node:module";
import {
  DriverLoader,
  ProviderCatalog,
  type DefaultValueConverter,
  type DeterministicTypeConverter,
  type IDriver,
  type IStatement,
  type SpecField,
  type StatementTextRenderer,
} from "@datasources/core";

const require = createRequire(import.meta.url);

export type SqlDialect = string;

export type SqlGenerateDriver = IDriver & {
  quoteLeft: string;
  quoteRight: string;
  typeConverter: DeterministicTypeConverter;
  defaultValueConverter: DefaultValueConverter;
  statementRenderer: StatementTextRenderer;
};

export const dialectDriver = (dialect: string): SqlGenerateDriver => {
  const catalog = ProviderCatalog.default[dialect];
  if (catalog === undefined) throw new Error(`Unknown SQL dialect "${dialect}".`);
  const driver = DriverLoader.load(
    new ProviderCatalog(catalog.name, require.resolve(catalog.location), catalog.module),
  );
  if (
    driver.typeConverter == null ||
    driver.defaultValueConverter == null ||
    driver.statementRenderer == null
  ) {
    throw new Error(`"${dialect}" is not a SQL dialect.`);
  }
  return driver as SqlGenerateDriver;
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
