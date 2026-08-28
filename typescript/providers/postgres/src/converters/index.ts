import {
  TypeConverter,
  sqlProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const postgresTypes: ProviderTypes = sqlProviderTypes(
  {
  VARCHAR: 'TEXT',
  CHAR: 'TEXT',
  INT2: 'INTEGER',
  INT4: 'INTEGER',
  INT8: 'INTEGER',
  FLOAT4: 'REAL',
  FLOAT8: 'REAL',
  DECIMAL: 'NUMERIC',
  TIMESTAMPTZ: 'DATETIME',
  TIMESTAMP: 'DATETIME',
  DATE: 'DATETIME',
  BYTEA: 'BLOB',
},
  { booleanStorage: "native" },
);

export const registerConverters = () =>
  TypeConverter.use("postgres", postgresTypes);

registerConverters();
