import { DatasourceError } from "@datasources/core";

const affinity: Record<string, string> = {
  int: "INT",
  uuid: "CHAR(36)",
  string: "TEXT",
  bool: "TINYINT",
  decimal: "DECIMAL(18,4)",
  float: "DOUBLE",
  datetime: "TEXT",
  binary: "BLOB",
};

export const MysqlColumnType = {
  map: (canonical: string): string =>
    affinity[canonical] ??
    (() => {
      throw new DatasourceError(`Unknown mysql type "${canonical}"`);
    })(),
};
