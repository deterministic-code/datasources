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
import { PostgresConnection } from "./connection.ts";
import { PostgresTransaction } from "./transaction.ts";
import { PostgresRowTypeConverters } from "./converters/PostgresRowTypeConverters.ts";

export class PostgresDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = PostgresRowTypeConverters;
  connection = (options?: DatasourceOptions): IConnection =>
    PostgresConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new PostgresTransaction(connection as PostgresConnection, isolation);
}
