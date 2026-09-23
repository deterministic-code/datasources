import type {
  DatasourceOptions,
  ICommand,
  IConnection,
  IDriver,
  ITransaction,
  IsolationLevel,
} from "@datasources/core";
import type { Cell, TypeConverter } from "@datasources/core";
import { MemoryConnection } from "./connection.ts";
import { MemoryTransaction } from "./transaction.ts";

export class MemoryProvider implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  connection = (options?: DatasourceOptions): IConnection =>
    MemoryConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new MemoryTransaction(connection as MemoryConnection, isolation);
}
