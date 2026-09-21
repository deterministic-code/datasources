import { DatasourceError } from "@datasources/core";

const affinity: Record<string, string> = {
  int: "INTEGER",
  uuid: "TEXT",
  string: "TEXT",
  bool: "INTEGER",
  decimal: "NUMERIC",
  float: "REAL",
  datetime: "TEXT",
  binary: "BLOB",
};

export const SqliteColumnType = {
  map: (canonical: string): string =>
    affinity[canonical] ??
    (() => {
      throw new DatasourceError(`Unknown sqlite type "${canonical}"`);
    })(),
};
