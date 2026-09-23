import type {
  Cell,
  DatasourceOptions,
  ICommand,
  IConnection,
  IDriver,
  ITransaction,
  IsolationLevel,
  TypeConverter,
} from "@datasources/core";
import { SqliteConnection } from "./connection.ts";
import { SqliteTransaction } from "./transaction.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";

export class SqliteDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = SqliteRowTypeConverters;
  connection = (options?: DatasourceOptions): IConnection =>
    SqliteConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new SqliteTransaction(connection as SqliteConnection, isolation);
}
