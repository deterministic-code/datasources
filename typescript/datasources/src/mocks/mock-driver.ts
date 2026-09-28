import { columnType as identityColumnType, generatedId as identityGeneratedId } from "../IDriver.ts";
import type { DatasourceOptions } from "../DatasourceOptions.ts";
import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IDriver } from "../IDriver.ts";
import type { ITransaction } from "../ITransaction.ts";
import type { IsolationLevel } from "../IsolationLevel.ts";
import type { Cell } from "../converters/Cell.ts";
import type { TypeConverter } from "../converters/TypeConverter.ts";
import { MockCommand } from "./mock-command.ts";
import { MockConnection } from "./mock-connection.ts";
import { MockTransaction } from "./mock-transaction.ts";

export class MockDriver implements IDriver {
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  generatedId = identityGeneratedId;
  columnType = identityColumnType;
  connection = (_options?: DatasourceOptions): IConnection => new MockConnection();
  command = (connection: IConnection): ICommand => new MockCommand(connection);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "serializable",
  ): ITransaction => {
    const txn = new MockTransaction(connection);
    txn.isolation = isolation;
    return txn;
  };
}
