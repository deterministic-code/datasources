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
import { JsonConnection } from "./connection.ts";
import { JsonTransaction } from "./transaction.ts";

export class JsonDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Value, Value>> = {};
  columnType = identityColumnType;
  defaultValueConverter = emptyDefaultValueConverter;
  connection = (options?: DatasourceOptions): IConnection =>
    JsonConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new JsonTransaction(connection as JsonConnection, isolation);
}
