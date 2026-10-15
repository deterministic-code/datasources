import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { ITransaction } from "./ITransaction.ts";
import type { IsolationLevel } from "./IsolationLevel.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { Cell } from "./converters/Cell.ts";
import type { DefaultValueConverter } from "./converters/DefaultValueConverter.ts";
import type { DeterministicTypeConverter } from "./converters/DeterministicTypeConverter.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { Information } from "./Information.ts";
import type { StatementTextRenderer } from "./statements/StatementTextRenderer.ts";

export const columnType = (name: string): string => name;

export interface IDriver {
  connection(options?: DatasourceOptions): IConnection;
  command(connection: IConnection): ICommand;
  transaction(
    connection: IConnection,
    isolation?: IsolationLevel,
  ): ITransaction;
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>>;
  columnType(name: string): string;
  deterministicTypeConverter?: DeterministicTypeConverter | null;
  defaultValueConverter?: DefaultValueConverter | null;
  statementRenderer?: StatementTextRenderer | null;
  information?: Information | null;
}
