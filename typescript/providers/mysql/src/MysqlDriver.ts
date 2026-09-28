import {
  DatasourceError,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { MysqlConnection } from "./connection.ts";
import { MysqlTransaction } from "./transaction.ts";
import { MysqlRowTypeConverters } from "./converters/MysqlRowTypeConverters.ts";

export class MysqlDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = MysqlRowTypeConverters;
  columnType = (name: string): string =>
    ({
      int: "INT",
      uuid: "CHAR(36)",
      string: "TEXT",
      datetime: "TEXT",
      bool: "TINYINT",
      decimal: "DECIMAL(18,4)",
      float: "DOUBLE",
      binary: "BLOB",
    }[name] ??
    (() => {
      throw new DatasourceError(`Unknown mysql type "${name}"`);
    })());
  connection = (options?: DatasourceOptions): IConnection =>
    MysqlConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new MysqlTransaction(connection as MysqlConnection, isolation);
}
