import {
  CreateTableStatement,
  DatasourceError,
  DeleteStatement,
  InsertStatement,
  QueryStatement,
  StoredProcedureStatement,
  UpdateStatement,
  type Field,
  type IWhere,
} from "@datasources/core";

export type MysqlColumn = { name: string; type: string; id?: boolean };

const affinity: Record<string, string> = {
  int: "INT",
  uuid: "CHAR(36)",
  string: "TEXT",
  bool: "TINYINT",
  decimal: "DECIMAL(18,4)",
  float: "DOUBLE",
  datetime: "TEXT",
  binary: "BLOB",
};

export const MysqlStatementBuilder = {
  quote: (id: string): string => `\`${id.replaceAll("`", "``")}\``,

  columnType: (canonical: string): string =>
    affinity[canonical] ??
    (() => {
      throw new DatasourceError(`Unknown mysql type "${canonical}"`);
    })(),

  createTable: (table: string, columns: MysqlColumn[]) =>
    new CreateTableStatement(
      MysqlStatementBuilder.quote(table),
      columns.map((column) => ({
        name: MysqlStatementBuilder.quote(column.name),
        type: column.id
          ? `${MysqlStatementBuilder.columnType(column.type)} PRIMARY KEY`
          : MysqlStatementBuilder.columnType(column.type),
      })),
    ),

  createQuery: (table: string, fields: string[], where?: IWhere) =>
    new QueryStatement(
      MysqlStatementBuilder.quote(table),
      fields.map(MysqlStatementBuilder.quote),
      where,
    ),

  createInsert: (table: string, fields: Field[]) =>
    new InsertStatement(MysqlStatementBuilder.quote(table), fields),

  createStoredProcedure: (name: string, parameters: Field[] = []) =>
    new StoredProcedureStatement(MysqlStatementBuilder.quote(name), parameters),

  createUpdate: (table: string, fields: Field[], where?: IWhere) =>
    new UpdateStatement(MysqlStatementBuilder.quote(table), fields, where),

  createDelete: (table: string, where?: IWhere) =>
    new DeleteStatement(MysqlStatementBuilder.quote(table), where),
};
