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
import { SqliteConnection } from "./connection.ts";
import { SqliteCommand } from "./command.ts";
import { SqliteTransaction } from "./transaction.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";
import { SqliteStatementRenderer } from "./SqliteStatementRenderer.ts";
import { columnTypes } from "./columnTypes.ts";
import { procedureSpecs } from "./procedureSpecs.ts";
import { applyKeywords } from "./applyKeywords.ts";
import { TypeMap } from "./TypeMap.ts";
import { DefaultValueConverter } from "./DefaultValueConverter.ts";

export { columnTypes };

let statements: StatementTextRenderer | undefined;
const renderer = (): StatementTextRenderer =>
  (statements ??= new StatementTextRenderer(t => new SqliteStatementRenderer(t)));
const types = new TypeMap({
  string: "TEXT",
  character: "TEXT",
  integer: "INTEGER",
  unsignedinteger: "INTEGER",
  smallinteger: "INTEGER",
  unsignedsmallinteger: "INTEGER",
  biginteger: "INTEGER",
  unsignedbiginteger: "INTEGER",
  float: "REAL",
  decimal: "NUMERIC",
  boolean: "BOOLEAN",
  datetime: "TEXT",
  binary: "BLOB",
  uuid: "TEXT",
  reference: "INTEGER",
});
const defaults = new DefaultValueConverter({
  newId: () =>
    "(lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6))))",
  now: () => `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))`,
  utcNow: () => `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))`,
  booleanOn: "1",
  booleanOff: "0",
  hex: (arg) => `X'${arg}'`,
  hexReverse: /^X'(.*)'$/,
});

export class SqliteDriver implements IDriver {
  static readonly procedurePayloadType = "JSON";
  static procedureSpecs = procedureSpecs;
  static applyKeywords = applyKeywords;
  static quote = (name: string): string => `"${name}"`;
  static nativeType = (column: Column): string =>
    column.field ? types.convertField(column.field) : columnTypes[column.type] ?? column.type;
  static nativeDefault = (column: Column): string =>
    column.default == null
      ? ""
      : column.field === undefined
        ? column.default.value
        : defaults.convert(column.field.type, column.default.value) ?? column.default.value;
  static utcNow = (): string => defaults.convert("datetime", "UtcNow")!;
  static nativeParam = (parameter: RoutineParameter): string =>
    parameter.type === "payload"
      ? SqliteDriver.procedurePayloadType
      : parameter.field
        ? types.convertField(parameter.field)
        : types.dictionary[parameter.type] ?? parameter.type;

  get statementRenderer(): StatementTextRenderer {
    return renderer();
  }
  readonly information: Information = {
    descriptions: [
      "Embedded SQL database for local development and tests.",
      "Full connection, command, transaction, and repository support.",
    ],
    overrides: [
      "Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}",
      "Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)",
    ],
    changeLogs: [],
  };
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = SqliteRowTypeConverters;
  readonly quoteLeft = '"';
  readonly quoteRight = '"';
  readonly typeConverter = types;
  readonly defaultValueConverter = defaults;
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown sqlite type "${name}"`);
    })();
  connection = (options?: DatasourceOptions): IConnection =>
    SqliteConnection.from(options);
  command = (connection: IConnection): ICommand =>
    new SqliteCommand(connection as SqliteConnection, renderer());
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new SqliteTransaction(connection as SqliteConnection, isolation);
}
