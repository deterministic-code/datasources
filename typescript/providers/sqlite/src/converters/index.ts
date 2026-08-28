import {
  TypeConverter,
  sqlProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const sqliteTypes: ProviderTypes = sqlProviderTypes(
  {
  CHAR: 'TEXT',
  VARCHAR: 'TEXT',
  TINYINT: 'INTEGER',
  SMALLINT: 'INTEGER',
  INT: 'INTEGER',
  BIGINT: 'INTEGER',
  FLOAT: 'REAL',
  DOUBLE: 'REAL',
  DATE: 'DATETIME',
},
  { booleanStorage: "integer" },
);

export const registerConverters = () =>
  TypeConverter.use("sqlite", sqliteTypes);

registerConverters();
