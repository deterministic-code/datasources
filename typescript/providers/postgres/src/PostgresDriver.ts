import {
  DatasourceError,
  StatementTextRenderer,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { PostgresConnection } from "./connection.ts";
import { PostgresCommand } from "./command.ts";
import { PostgresTransaction } from "./transaction.ts";
import { PostgresRowTypeConverters } from "./converters/PostgresRowTypeConverters.ts";
import { PostgresStatementRenderer } from "./PostgresStatementRenderer.ts";
import { columnTypes } from "./columnTypes.ts";

export { columnTypes };

const statements = new StatementTextRenderer(t => new PostgresStatementRenderer(t));

export class PostgresDriver implements IDriver {
  readonly statementRenderer = statements;
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = PostgresRowTypeConverters;
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown postgres type "${name}"`);
    })();
  connection = (options?: DatasourceOptions): IConnection =>
    PostgresConnection.from(options);
  command = (connection: IConnection): ICommand =>
    new PostgresCommand(connection as PostgresConnection, statements);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new PostgresTransaction(connection as PostgresConnection, isolation);
}
