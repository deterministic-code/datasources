import {
  columnType as identityColumnType,
  DefaultValueConverter,
  DeterministicTypeConverter,
  StatementTextRenderer,
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
import { OracleConnection } from "./connection.ts";
import { OracleStatementRenderer } from "./OracleStatementRenderer.ts";

const statements = new StatementTextRenderer(t => new OracleStatementRenderer(t));

export class OracleDriver implements IDriver {
  readonly statementRenderer = statements;
  rowTypeConverters: Record<string, TypeConverter<Cell, Cell>> = {};
  readonly quoteLeft = '"';
  readonly quoteRight = '"';
  readonly typeConverter = new DeterministicTypeConverter(
    {
      string: "CLOB",
      character: "CHAR(1)",
      number: "NUMBER(10)",
      integer: "NUMBER(10)",
      unsignedinteger: "NUMBER(10)",
      smallinteger: "NUMBER(5)",
      unsignedsmallinteger: "NUMBER(5)",
      biginteger: "NUMBER(19)",
      unsignedbiginteger: "NUMBER(20)",
      float: "BINARY_DOUBLE",
      decimal: "NUMBER",
      boolean: "NUMBER(1)",
      datetime: "TIMESTAMP",
      binary: "BLOB",
      uuid: "VARCHAR2(36)",
      reference: "NUMBER(10)",
    },
    {
      dialectName: "oracle",
      stringUnlimited: "CLOB",
      stringSized: (n: number) => `VARCHAR2(${n})`,
      character: "CHAR",
      decimal: { kind: "required", type: "NUMBER" },
      binaryUnlimited: "BLOB",
      binarySized: (n: number) => `RAW(${n})`,
    },
  );
  readonly defaultValueConverter = new DefaultValueConverter({
    newId: () => "SYS_GUID()",
    now: () => "LOCALTIMESTAMP",
    utcNow: () => "SYS_EXTRACT_UTC(SYSTIMESTAMP)",
    booleanOn: "1",
    booleanOff: "0",
    hex: (arg: string) => `HEXTORAW('${arg}')`,
    hexReverse: /^HEXTORAW\('(.*)'\)$/,
  });
  columnType = identityColumnType;
  connection = (options?: DatasourceOptions): IConnection =>
    OracleConnection.from(options);
  command = (connection: IConnection): ICommand => connection.createCommand();
  transaction = (
    connection: IConnection,
    _isolation?: IsolationLevel,
  ): ITransaction => new StubTransaction(connection.provider, connection);
}
