import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { ITransaction } from "./ITransaction.ts";
import type { IsolationLevel } from "./IsolationLevel.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { Value } from "./converters/Value.ts";
import type { IDefaultValueConverter } from "./converters/IDefaultValueConverter.ts";
import type { IMapping } from "./converters/IMapping.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { Information } from "./Information.ts";
import type { StatementTextRenderer } from "./statements/StatementTextRenderer.ts";
import { emptyDefaultValueConverter } from "./converters/EmptyDefaultValueConverter.ts";

export { emptyDefaultValueConverter };

export const columnType = (name: string): string => name;

export interface IDriver {
  connection(options?: DatasourceOptions): IConnection;
  command(connection: IConnection): ICommand;
  transaction(
    connection: IConnection,
    isolation?: IsolationLevel,
  ): ITransaction;
  rowTypeConverters: Record<string, TypeConverter<Value, Value>>;
  columnType(name: string): string;
  typeConverter?: IMapping<string, string> | null;
  defaultValueConverter: IDefaultValueConverter;
  statementRenderer?: StatementTextRenderer | null;
  information?: Information | null;
}
