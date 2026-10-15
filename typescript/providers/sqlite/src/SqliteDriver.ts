import {
  DatasourceError,
  DefaultValueConverter,
  DeterministicTypeConverter,
  StatementTextRenderer,
  type Cell,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDriver,
  type Information,
  type ITransaction,
  type IsolationLevel,
  type TypeConverter,
} from "@datasources/core";
import { SqliteConnection } from "./connection.ts";
import { SqliteCommand } from "./command.ts";
import { SqliteTransaction } from "./transaction.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";
import { SqliteStatementRenderer } from "./SqliteStatementRenderer.ts";
import { columnTypes } from "./columnTypes.ts";

export { columnTypes };

const statements = new StatementTextRenderer(t => new SqliteStatementRenderer(t));

export class SqliteDriver implements IDriver {
  readonly statementRenderer = statements;
  readonly information: Information = {
    descriptions: [
      "Embedded SQL database for local development and tests.",
      "Full connection, command, transaction, and repository support.",
    ],
    overrides: [
      "Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}",
      "Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)",
    ],
    changeLogs: [],
  };
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = SqliteRowTypeConverters;
  readonly quoteLeft = '"';
  readonly quoteRight = '"';
  readonly typeConverter = new DeterministicTypeConverter(
    {
      string: "TEXT",
      character: "TEXT",
      number: "INTEGER",
      integer: "INTEGER",
      unsignedinteger: "INTEGER",
      smallinteger: "INTEGER",
      unsignedsmallinteger: "INTEGER",
      biginteger: "INTEGER",
      unsignedbiginteger: "INTEGER",
      float: "REAL",
      decimal: "NUMERIC",
      boolean: "BOOLEAN",
      datetime: "TEXT",
      binary: "BLOB",
      uuid: "TEXT",
      reference: "INTEGER",
    },
    {
      dialectName: "sqlite",
      stringUnlimited: "TEXT",
      stringSized: (n) => `VARCHAR(${n})`,
    },
  );
  readonly defaultValueConverter = new DefaultValueConverter({
    newId: () => "",
    now: () => `(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))`,
    utcNow: () => `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))`,
    booleanOn: "1",
    booleanOff: "0",
    hex: (arg) => `X'${arg}'`,
    hexReverse: /^X'(.*)'$/,
  });
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown sqlite type "${name}"`);
    })();
  connection = (options?: DatasourceOptions): IConnection =>
    SqliteConnection.from(options);
  command = (connection: IConnection): ICommand =>
    new SqliteCommand(connection as SqliteConnection, statements);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new SqliteTransaction(connection as SqliteConnection, isolation);
}
