import {
  generatedId as identityGeneratedId,
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
import { SqlServerConnection } from "./connection.ts";

export class SqlServerProvider implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  generatedId = identityGeneratedId;
  connection = (options?: DatasourceOptions): IConnection =>
    SqlServerConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
