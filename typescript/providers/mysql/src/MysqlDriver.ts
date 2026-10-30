import {
  DatasourceError,
  DefaultValueConverter,
  sqlStringLiteral,
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
} from "@deterministic-code/datasources";
import { MysqlConnection } from "./connection.ts";
import { MysqlCommand } from "./command.ts";
import { MysqlTransaction } from "./transaction.ts";
import { MysqlRowTypeConverters } from "./converters/MysqlRowTypeConverters.ts";
import { MysqlStatementRenderer } from "./MysqlStatementRenderer.ts";
import { columnTypes } from "./columnTypes.ts";

export { columnTypes };

const statements = new StatementTextRenderer(t => new MysqlStatementRenderer(t));

export class MysqlDriver implements IDriver {
  readonly statementRenderer = statements;
  readonly information: Information = {
    descriptions: [
      "MySQL / MariaDB server driver.",
      "Full connection, command, transaction, and repository support.",
    ],
    overrides: [
      "Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}",
      "Overrides CreateTableStatement identity to AUTO_INCREMENT",
      "Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)",
      "Overrides CreateCreateStoredProcedureStatement to PROCEDURE Create{Plural}",
      "Overrides CreateReadStoredProcedureStatement to PROCEDURE Read{Plural}By{Properties}",
      "Overrides CreateUpdateStoredProcedureStatement to PROCEDURE Update{Plural}By{Properties}",
      "Overrides CreateDeleteStoredProcedureStatement to PROCEDURE Delete{Plural}By{Properties}",
      "Overrides CreateBulkCreateStoredProcedureStatement to PROCEDURE BulkCreate{Plural}",
      "Overrides CreateBulkUpdateStoredProcedureStatement to PROCEDURE BulkUpdate{Plural}",
      "Overrides CreateBulkDeleteStoredProcedureStatement to PROCEDURE BulkDelete{Plural}",
    ],
    changeLogs: [],
  };
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = MysqlRowTypeConverters;
  readonly quoteLeft = "`";
  readonly quoteRight = "`";
  readonly typeConverter = new DeterministicTypeConverter(
    {
      string: "TEXT",
      character: "CHAR(1)",
      integer: "INT",
      unsignedinteger: "INT UNSIGNED",
      smallinteger: "SMALLINT",
      unsignedsmallinteger: "SMALLINT UNSIGNED",
      biginteger: "BIGINT",
      unsignedbiginteger: "BIGINT UNSIGNED",
      float: "DOUBLE",
      decimal: "DECIMAL(10, 2)",
      boolean: "TINYINT(1)",
      datetime: "DATETIME(3)",
      binary: "BLOB",
      uuid: "CHAR(36)",
      reference: "INT",
    },
    {
      dialectName: "mysql",
      stringUnlimited: "TEXT",
      stringSized: (n) => (n <= 16383 ? `VARCHAR(${n})` : "LONGTEXT"),
      character: "CHAR",
      decimal: { kind: "optional", type: "DECIMAL", fallback: "DECIMAL(10, 2)" },
      binaryUnlimited: "BLOB",
      binarySized: (n) => (n <= 65535 ? `VARBINARY(${n})` : "LONGBLOB"),
    },
  );
  readonly defaultValueConverter = new DefaultValueConverter({
    newId: () => `(UUID())`,
    now: () => `(NOW(3))`,
    utcNow: () => `(UTC_TIMESTAMP(3))`,
    booleanOn: "1",
    booleanOff: "0",
    hex: (arg) => `(X'${arg}')`,
    hexReverse: /^(?:\(X'(.*)'\)|X'(.*)')$/,
    quote: (arg) => `(${sqlStringLiteral(arg)})`,
    quoteReverse: /^(?:\('(.*)'\)|'(.*)')$/,
  });
  columnType = (name: string): string =>
    columnTypes[name] ??
    (() => {
      throw new DatasourceError(`Unknown mysql type "${name}"`);
    })();
  connection = (options?: DatasourceOptions): IConnection =>
    MysqlConnection.from(options);
  command = (connection: IConnection): ICommand =>
    new MysqlCommand(connection as MysqlConnection, statements);
  transaction = (
    connection: IConnection,
    isolation: IsolationLevel = "readCommitted",
  ): ITransaction =>
    new MysqlTransaction(connection as MysqlConnection, isolation);
}
