import {
  columnType as identityColumnType,
  DefaultValueConverter,
  DeterministicTypeConverter,
  StubTransaction,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { SqlServerConnection } from "./connection.ts";

export class SqlServerDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  readonly quoteLeft = "[";
  readonly quoteRight = "]";
  readonly deterministicTypeConverter = new DeterministicTypeConverter(
    {
      string: "NVARCHAR(MAX)",
      character: "NCHAR(1)",
      number: "INT",
      integer: "INT",
      unsignedinteger: "BIGINT",
      smallinteger: "SMALLINT",
      unsignedsmallinteger: "INT",
      biginteger: "BIGINT",
      unsignedbiginteger: "DECIMAL(20)",
      float: "FLOAT",
      decimal: "DECIMAL",
      boolean: "BIT",
      datetime: "DATETIME2",
      binary: "VARBINARY(MAX)",
      uuid: "UNIQUEIDENTIFIER",
      reference: "INT",
    },
    {
      dialectName: "sqlserver",
      stringUnlimited: "NVARCHAR(MAX)",
      stringSized: (n: number) => `NVARCHAR(${n})`,
      character: "NCHAR",
      decimal: { kind: "required", type: "DECIMAL" },
      binaryUnlimited: "VARBINARY(MAX)",
      binarySized: (n: number) => `VARBINARY(${n})`,
      occBinary: "BINARY(8)",
      rowVersion: "ROWVERSION",
    },
  );
  readonly defaultValueConverter = new DefaultValueConverter({
    newId: () => "NEWID()",
    now: () => "GETDATE()",
    utcNow: () => "GETUTCDATE()",
    booleanOn: "1",
    booleanOff: "0",
    hex: (arg: string) => `0x${arg}`,
    hexReverse: /^0x(.*)$/,
  });
  columnType = identityColumnType;
  connection = (options?: DatasourceOptions): IConnection =>
    SqlServerConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
