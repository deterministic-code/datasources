import {
  columnType as identityColumnType,
  generatedId as identityGeneratedId,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { JsonConnection } from "./connection.ts";
import { JsonTransaction } from "./transaction.ts";

export class JsonProvider implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  generatedId = identityGeneratedId;
  columnType = identityColumnType;
  connection = (options?: DatasourceOptions): IConnection =>
    JsonConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new JsonTransaction(connection as JsonConnection, isolation);
}
