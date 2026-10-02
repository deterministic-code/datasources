import {
  DatasourceError,
  StatementTextRenderer,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type Information,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { SqliteConnection } from "./connection.ts";
import { SqliteCommand } from "./command.ts";
import { SqliteTransaction } from "./transaction.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";
import { SqliteStatementRenderer } from "./SqliteStatementRenderer.ts";
import { columnTypes } from "./columnTypes.ts";

export { columnTypes };

const statements = new StatementTextRenderer(t => new SqliteStatementRenderer(t));

export class SqliteDriver implements IDriver {
  readonly statementRenderer = statements;
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
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown sqlite type "${name}"`);
    })();
  connection = (options?: DatasourceOptions): IConnection =>
    SqliteConnection.from(options);
  command = (connection: IConnection): ICommand =>
    new SqliteCommand(connection as SqliteConnection, statements);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new SqliteTransaction(connection as SqliteConnection, isolation);
}
