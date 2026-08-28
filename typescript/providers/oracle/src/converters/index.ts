import {
  TypeConverter,
  sqlProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const oracleTypes: ProviderTypes = sqlProviderTypes(
  {
  VARCHAR2: 'TEXT',
  NVARCHAR2: 'TEXT',
  CHAR: 'TEXT',
  CLOB: 'TEXT',
  NUMBER: 'NUMERIC',
  BINARY_FLOAT: 'REAL',
  BINARY_DOUBLE: 'REAL',
  TIMESTAMP: 'DATETIME',
  DATE: 'DATETIME',
  RAW: 'BLOB',
},
  { booleanStorage: "integer" },
);

export const registerConverters = () =>
  TypeConverter.use("oracle", oracleTypes);

registerConverters();
