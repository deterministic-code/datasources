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
import { MysqlConnection } from "./connection.ts";
import { MysqlCommand } from "./command.ts";
import { MysqlTransaction } from "./transaction.ts";
import { MysqlRowTypeConverters } from "./converters/MysqlRowTypeConverters.ts";
import { MysqlStatementRenderer } from "./MysqlStatementRenderer.ts";
import { columnTypes } from "./columnTypes.ts";

export { columnTypes };

const statements = new StatementTextRenderer(t => new MysqlStatementRenderer(t));

export class MysqlDriver implements IDriver {
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
    ],
    changeLogs: [],
  };
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = MysqlRowTypeConverters;
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
