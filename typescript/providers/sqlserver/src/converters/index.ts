import {
  TypeConverter,
  sqlProviderTypes,
  type ProviderTypes,
} from "@datasources/core";

export const sqlserverTypes: ProviderTypes = sqlProviderTypes(
  {
  NVARCHAR: 'TEXT',
  VARCHAR: 'TEXT',
  NCHAR: 'TEXT',
  CHAR: 'TEXT',
  INT: 'INTEGER',
  BIGINT: 'INTEGER',
  SMALLINT: 'INTEGER',
  TINYINT: 'INTEGER',
  FLOAT: 'REAL',
  DECIMAL: 'NUMERIC',
  BIT: 'BOOLEAN',
  DATETIME2: 'DATETIME',
  DATE: 'DATETIME',
  VARBINARY: 'BLOB',
  UNIQUEIDENTIFIER: 'UUID',
},
  { booleanStorage: "integer" },
);

export const registerConverters = () =>
  TypeConverter.use("sqlserver", sqlserverTypes);

registerConverters();
