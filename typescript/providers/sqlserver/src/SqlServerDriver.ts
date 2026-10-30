import {
  columnType as identityColumnType,
  DefaultValueConverter,
  DeterministicTypeConverter,
  StatementTextRenderer,
  StubTransaction,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type Information,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@deterministic-code/datasources";
import { SqlServerConnection } from "./connection.ts";
import { SqlServerStatementRenderer } from "./SqlServerStatementRenderer.ts";

const statements = new StatementTextRenderer(t => new SqlServerStatementRenderer(t));

export class SqlServerDriver implements IDriver {
  readonly statementRenderer = statements;
  readonly information: Information = {
    descriptions: ["SQL Server driver."],
    overrides: [
      "Overrides CreateCreateStoredProcedureStatement to PROCEDURE Create{Plural}",
      "Overrides CreateReadStoredProcedureStatement to PROCEDURE Read{Plural}By{Properties}",
      "Overrides CreateUpdateStoredProcedureStatement to PROCEDURE Update{Plural}By{Properties}",
      "Overrides CreateDeleteStoredProcedureStatement to PROCEDURE Delete{Plural}By{Properties}",
      "Overrides CreateBulkCreateStoredProcedureStatement to PROCEDURE BulkCreate{Plural}",
      "Overrides CreateBulkUpdateStoredProcedureStatement to PROCEDURE BulkUpdate{Plural}",
      "Overrides CreateBulkDeleteStoredProcedureStatement to PROCEDURE BulkDelete{Plural}",
    ],
    changeLogs: [],
  };
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  readonly quoteLeft = "[";
  readonly quoteRight = "]";
  readonly typeConverter = new DeterministicTypeConverter(
    {
      string: "NVARCHAR(MAX)",
      character: "NCHAR(1)",
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
