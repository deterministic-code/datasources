import {
  columnType as identityColumnType,
  StatementTextRenderer,
  StubTransaction,
  type Cell,
  type Column,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type Information,
  type ITransaction,
  type IsolationLevel,
  type RoutineParameter,
  type TypeConverter,
} from "@deterministic-code/datasources";
import { SqlServerConnection } from "./connection.ts";
import { SqlServerStatementRenderer } from "./SqlServerStatementRenderer.ts";
import { procedureSpecs } from "./procedureSpecs.ts";
import { applyKeywords } from "./applyKeywords.ts";
import { TypeMap } from "./TypeMap.ts";
import { DefaultValueConverter } from "./DefaultValueConverter.ts";

const statements = new StatementTextRenderer(t => new SqlServerStatementRenderer(t));
const types = new TypeMap({
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
});
const defaults = new DefaultValueConverter({
  newId: () => "NEWID()",
  now: () => "GETDATE()",
  utcNow: () => "GETUTCDATE()",
  booleanOn: "1",
  booleanOff: "0",
  hex: (arg: string) => `0x${arg}`,
  hexReverse: /^0x(.*)$/,
});

export class SqlServerDriver implements IDriver {
  static readonly procedurePayloadType = "NVARCHAR(MAX)";
  static procedureSpecs = procedureSpecs;
  static applyKeywords = applyKeywords;
  static quote = (name: string): string => `[${name}]`;
  static nativeType = (column: Column): string =>
    column.field ? types.convertField(column.field) : column.type;
  static nativeDefault = (column: Column): string =>
    column.default == null
      ? ""
      : column.field === undefined
        ? column.default.value
        : defaults.convert(column.field.type, column.default.value) ?? column.default.value;
  static utcNow = (): string => defaults.convert("datetime", "UtcNow") ?? "";
  static nativeParam = (parameter: RoutineParameter): string =>
    parameter.type === "payload"
      ? SqlServerDriver.procedurePayloadType
      : parameter.field
        ? types.convertField(parameter.field)
        : types.dictionary[parameter.type] ?? parameter.type;

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
  readonly typeConverter = types;
  readonly defaultValueConverter = defaults;
  columnType = identityColumnType;
  connection = (options?: DatasourceOptions): IConnection =>
    SqlServerConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
