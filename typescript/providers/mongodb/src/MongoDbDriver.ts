import {
  columnType as identityColumnType,
  emptyDefaultValueConverter,
  StubTransaction,
  type Value,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@deterministic-code/datasources";
import { MongoDbConnection } from "./connection.ts";

export class MongoDbDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Value, Value>> = {};
  columnType = identityColumnType;
  defaultValueConverter = emptyDefaultValueConverter;
  connection = (options?: DatasourceOptions): IConnection =>
    MongoDbConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
