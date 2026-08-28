import {
  TypeConverter,
  sqlProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const mysqlTypes: ProviderTypes = sqlProviderTypes(
  {
  VARCHAR: 'TEXT',
  CHAR: 'TEXT',
  TINYTEXT: 'TEXT',
  MEDIUMTEXT: 'TEXT',
  LONGTEXT: 'TEXT',
  TINYINT: 'INTEGER',
  SMALLINT: 'INTEGER',
  MEDIUMINT: 'INTEGER',
  INT: 'INTEGER',
  BIGINT: 'INTEGER',
  FLOAT: 'REAL',
  DOUBLE: 'REAL',
  DECIMAL: 'NUMERIC',
  "TINYINT(1)": "BOOLEAN",
  TIMESTAMP: "DATETIME",
  DATE: "DATETIME",
  BINARY: "BLOB",
  "CHAR(36)": "UUID",
},
  { booleanStorage: "integer" },
);

export const registerConverters = () =>
  TypeConverter.use("mysql", mysqlTypes);

registerConverters();
