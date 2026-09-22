import {
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
import { XmlConnection } from "./connection.ts";

export class XmlProvider implements IDriver {
  types: Record<string, TypeConverter<Cell, Cell>> = {};
  connection = (options?: DatasourceOptions): IConnection =>
    XmlConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
