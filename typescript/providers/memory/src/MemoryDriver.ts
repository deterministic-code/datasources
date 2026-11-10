import {
  columnType as identityColumnType,
  emptyDefaultValueConverter,
  type Value,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@deterministic-code/datasources";
import { MemoryConnection } from "./connection.ts";
import { MemoryTransaction } from "./transaction.ts";

export class MemoryDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Value, Value>> = {};
  columnType = identityColumnType;
  defaultValueConverter = emptyDefaultValueConverter;
  connection = (options?: DatasourceOptions): IConnection =>
    MemoryConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new MemoryTransaction(connection as MemoryConnection, isolation);
}
