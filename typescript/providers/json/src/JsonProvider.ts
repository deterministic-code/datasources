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
import { JsonConnection } from "./connection.ts";
import { JsonTransaction } from "./transaction.ts";

export class JsonProvider implements IDriver {
  types: Record<string, TypeConverter<Cell, Cell>> = {};
  connection = (options?: DatasourceOptions): IConnection =>
    JsonConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new JsonTransaction(connection as JsonConnection, isolation);
}
