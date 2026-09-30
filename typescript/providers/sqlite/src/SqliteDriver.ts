import {
  DatasourceError,
  RowTypeConverter,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { SqliteConnection } from "./connection.ts";
import { SqliteTransaction } from "./transaction.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";

export const columnTypes: Record<string, string> = {
  int: "INTEGER",
  uuid: "TEXT",
  string: "TEXT",
  bool: "BOOLEAN",
  decimal: "NUMERIC",
  float: "REAL",
  datetime: "TEXT",
  binary: "BLOB",
};

export class SqliteDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = SqliteRowTypeConverters;
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown sqlite type "${name}"`);
    })();
  rowConverter = (columns: Record<string, string>) => RowTypeConverter.from(this, columns);
  connection = (options?: DatasourceOptions): IConnection =>
    SqliteConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new SqliteTransaction(connection as SqliteConnection, isolation);
}
