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
import { OracleConnection } from "./connection.ts";
import { OracleStatementRenderer } from "./OracleStatementRenderer.ts";
import { applyKeywords } from "./applyKeywords.ts";
import { TypeMap } from "./TypeMap.ts";
import { DefaultValueConverter } from "./DefaultValueConverter.ts";

const statements = new StatementTextRenderer(t => new OracleStatementRenderer(t), undefined, undefined, undefined, applyKeywords);
const types = new TypeMap({
  string: "CLOB",
  character: "CHAR(1)",
  integer: "NUMBER(10)",
  unsignedinteger: "NUMBER(10)",
  smallinteger: "NUMBER(5)",
  unsignedsmallinteger: "NUMBER(5)",
  biginteger: "NUMBER(19)",
  unsignedbiginteger: "NUMBER(20)",
  float: "BINARY_DOUBLE",
  decimal: "NUMBER",
  boolean: "NUMBER(1)",
  datetime: "TIMESTAMP",
  binary: "BLOB",
  uuid: "VARCHAR2(36)",
  reference: "NUMBER(10)",
});
const defaults = new DefaultValueConverter({
  newId: () => "SYS_GUID()",
  now: () => "LOCALTIMESTAMP",
  utcNow: () => "SYS_EXTRACT_UTC(SYSTIMESTAMP)",
  booleanOn: "1",
  booleanOff: "0",
  hex: (arg: string) => `HEXTORAW('${arg}')`,
  hexReverse: /^HEXTORAW\('(.*)'\)$/,
});

export class OracleDriver implements IDriver {
  static readonly procedurePayloadType = "CLOB";
  static applyKeywords = applyKeywords;
  static quote = (name: string): string => `"${name}"`;
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
      ? OracleDriver.procedurePayloadType
      : parameter.field
        ? types.convertField(parameter.field)
        : types.dictionary[parameter.type] ?? parameter.type;

  readonly statementRenderer = statements;
  readonly information: Information = {
    descriptions: ["Oracle driver."],
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
  readonly quoteLeft = '"';
  readonly quoteRight = '"';
  readonly typeConverter = types;
  readonly defaultValueConverter = defaults;
  columnType = identityColumnType;
  connection = (options?: DatasourceOptions): IConnection =>
    OracleConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
