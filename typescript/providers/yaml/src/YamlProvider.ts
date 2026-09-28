import {
  columnType as identityColumnType,
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
import { YamlConnection } from "./connection.ts";

export class YamlProvider implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  generatedId = identityGeneratedId;
  columnType = identityColumnType;
  connection = (options?: DatasourceOptions): IConnection =>
    YamlConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
