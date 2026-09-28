import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { ITransaction } from "./ITransaction.ts";
import type { IsolationLevel } from "./IsolationLevel.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { Cell } from "./converters/Cell.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";

export const generatedId = (id: Cell): Cell => id;

export const columnType = (name: string): string => name;

export interface IDriver {
  connection(options?: DatasourceOptions): IConnection;
  command(connection: IConnection): ICommand;
  transaction(
    connection: IConnection,
    isolation?: IsolationLevel,
  ): ITransaction;
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>>;
  generatedId(id: Cell): Cell;
  columnType(name: string): string;
}
