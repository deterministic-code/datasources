import {
  DatasourceError,
  StatementBuilder,
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
    StatementBuilder.createTable(
      MysqlStatementBuilder.quote(table),
      columns.map((column) => ({
        name: MysqlStatementBuilder.quote(column.name),
        type: column.id
          ? `${MysqlStatementBuilder.columnType(column.type)} PRIMARY KEY`
          : MysqlStatementBuilder.columnType(column.type),
      })),
    ),

  createQuery: (table: string, fields: string[], where?: IWhere) =>
    StatementBuilder.createQuery(
      MysqlStatementBuilder.quote(table),
      fields.map(MysqlStatementBuilder.quote),
      where,
    ),

  createInsert: (table: string, fields: Field[]) =>
    StatementBuilder.createInsert(MysqlStatementBuilder.quote(table), fields),

  createUpdate: (table: string, fields: Field[], where?: IWhere) =>
    StatementBuilder.createUpdate(MysqlStatementBuilder.quote(table), fields, where),

  createDelete: (table: string, where?: IWhere) =>
    StatementBuilder.createDelete(MysqlStatementBuilder.quote(table), where),
};
