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
import { PostgresConnection } from "./connection.ts";
import { PostgresTransaction } from "./transaction.ts";
import { PostgresRowTypeConverters } from "./converters/PostgresRowTypeConverters.ts";

export const columnTypes: Record<string, string> = {
  int: "INTEGER",
  uuid: "UUID",
  string: "TEXT",
  bool: "BOOLEAN",
  decimal: "NUMERIC",
  float: "DOUBLE PRECISION",
  datetime: "TIMESTAMPTZ",
  binary: "BYTEA",
};

export class PostgresDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = PostgresRowTypeConverters;
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown postgres type "${name}"`);
    })();
  connection = (options?: DatasourceOptions): IConnection =>
    PostgresConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction =>
    new PostgresTransaction(connection as PostgresConnection, isolation);
}
