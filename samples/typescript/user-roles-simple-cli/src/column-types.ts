import { DatasourceError } from "@datasources/core";

const tables: Record<string, Record<string, string>> = {
  sqlite: {
    int: "INTEGER",
    uuid: "TEXT",
    string: "TEXT",
    bool: "INTEGER",
    decimal: "NUMERIC",
    float: "REAL",
    datetime: "TEXT",
    binary: "BLOB",
  },
  mysql: {
    int: "INT",
    uuid: "CHAR(36)",
    string: "TEXT",
    bool: "TINYINT",
    decimal: "DECIMAL(18,4)",
    float: "DOUBLE",
    datetime: "TEXT",
    binary: "BLOB",
  },
  postgres: {
    int: "INTEGER",
    uuid: "UUID",
    string: "TEXT",
    bool: "BOOLEAN",
    decimal: "NUMERIC",
    float: "DOUBLE PRECISION",
    datetime: "TIMESTAMPTZ",
    binary: "BYTEA",
  },
};

export const columnTypes = (provider: string): ((canonical: string) => string) => {
  const table = tables[provider] ?? (() => {
    throw new Error(`unknown provider ${provider}`);
  })();
  return (canonical) =>
    table[canonical] ??
    (() => {
      throw new DatasourceError(`Unknown ${provider} type "${canonical}"`);
    })();
};
