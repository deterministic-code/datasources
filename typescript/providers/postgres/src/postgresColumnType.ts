import { DatasourceError } from "@datasources/core";

const affinity: Record<string, string> = {
  int: "INTEGER",
  uuid: "UUID",
  string: "TEXT",
  bool: "BOOLEAN",
  decimal: "NUMERIC",
  float: "DOUBLE PRECISION",
  datetime: "TIMESTAMPTZ",
  binary: "BYTEA",
};

export const PostgresColumnType = {
  map: (canonical: string): string =>
    affinity[canonical] ??
    (() => {
      throw new DatasourceError(`Unknown postgres type "${canonical}"`);
    })(),
};
