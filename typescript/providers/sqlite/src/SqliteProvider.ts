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
import { SqliteTypes } from "./converters/SqliteTypes.ts";

export class SqliteProvider implements IDriver {
  types: Record<string, TypeConverter<Cell, Cell>> = SqliteTypes;
  connection = (options?: DatasourceOptions): IConnection =>
    SqliteConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new SqliteTransaction(connection as SqliteConnection, isolation);
}
