import {
  columnType as identityColumnType,
  StubTransaction,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { MongoDbConnection } from "./connection.ts";

export class MongoDbProvider implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  columnType = identityColumnType;
  connection = (options?: DatasourceOptions): IConnection =>
    MongoDbConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
