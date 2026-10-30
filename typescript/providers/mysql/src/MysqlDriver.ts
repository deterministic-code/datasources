import {
  DatasourceError,
  StatementTextRenderer,
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
import { MysqlConnection } from "./connection.ts";
import { MysqlCommand } from "./command.ts";
import { MysqlTransaction } from "./transaction.ts";
import { MysqlRowTypeConverters } from "./converters/MysqlRowTypeConverters.ts";
import { MysqlStatementRenderer } from "./MysqlStatementRenderer.ts";
import { columnTypes } from "./columnTypes.ts";
import { applyKeywords } from "./applyKeywords.ts";
import { TypeMap } from "./TypeMap.ts";
import { DefaultValueConverter, sqlStringLiteral } from "./DefaultValueConverter.ts";

export { columnTypes };

const statements = new StatementTextRenderer(t => new MysqlStatementRenderer(t), undefined, undefined, undefined, applyKeywords);
const types = new TypeMap({
  string: "TEXT",
  character: "CHAR(1)",
  integer: "INT",
  unsignedinteger: "INT UNSIGNED",
  smallinteger: "SMALLINT",
  unsignedsmallinteger: "SMALLINT UNSIGNED",
  biginteger: "BIGINT",
  unsignedbiginteger: "BIGINT UNSIGNED",
  float: "DOUBLE",
  decimal: "DECIMAL(10, 2)",
  boolean: "TINYINT(1)",
  datetime: "DATETIME(3)",
  binary: "BLOB",
  uuid: "CHAR(36)",
  reference: "INT",
});
const defaults = new DefaultValueConverter({
  newId: () => `(UUID())`,
  now: () => `(NOW(3))`,
  utcNow: () => `(UTC_TIMESTAMP(3))`,
  booleanOn: "1",
  booleanOff: "0",
  hex: (arg) => `(X'${arg}')`,
  hexReverse: /^(?:\(X'(.*)'\)|X'(.*)')$/,
  quote: (arg) => `(${sqlStringLiteral(arg)})`,
  quoteReverse: /^(?:\('(.*)'\)|'(.*)')$/,
});

export class MysqlDriver implements IDriver {
  static readonly procedurePayloadType = "JSON";
  static applyKeywords = applyKeywords;
  static quote = (name: string): string => `\`${name}\``;
  static nativeType = (column: Column): string =>
    column.field ? types.convertField(column.field) : columnTypes[column.type] ?? column.type;
  static nativeDefault = (column: Column): string =>
    column.default == null
      ? ""
      : column.field === undefined
        ? column.default.value
        : defaults.convert(column.field.type, column.default.value) ?? column.default.value;
  static utcNow = (): string => defaults.convert("datetime", "UtcNow") ?? "";
  static nativeParam = (parameter: RoutineParameter): string =>
    parameter.type === "payload"
      ? MysqlDriver.procedurePayloadType
      : parameter.field
        ? types.convertField(parameter.field)
        : types.dictionary[parameter.type] ?? parameter.type;

  readonly statementRenderer = statements;
  readonly information: Information = {
    descriptions: [
      "MySQL / MariaDB server driver.",
      "Full connection, command, transaction, and repository support.",
    ],
    overrides: [
      "Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}",
      "Overrides CreateTableStatement identity to AUTO_INCREMENT",
      "Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)",
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
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = MysqlRowTypeConverters;
  readonly quoteLeft = "`";
  readonly quoteRight = "`";
  readonly typeConverter = types;
  readonly defaultValueConverter = defaults;
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown mysql type "${name}"`);
    })();
  connection = (options?: DatasourceOptions): IConnection =>
    MysqlConnection.from(options);
  command = (connection: IConnection): ICommand =>
    new MysqlCommand(connection as MysqlConnection, statements);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new MysqlTransaction(connection as MysqlConnection, isolation);
}
