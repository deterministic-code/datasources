import {
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
import { MemoryConnection } from "./connection.ts";
import { MemoryTransaction } from "./transaction.ts";

export class MemoryProvider implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  generatedId = identityGeneratedId;
  connection = (options?: DatasourceOptions): IConnection =>
    MemoryConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new MemoryTransaction(connection as MemoryConnection, isolation);
}
