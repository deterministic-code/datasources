import {
  DatasourceError,
  StatementBuilder,
  type Field,
  type IWhere,
  type Join,
} from "@datasources/core";

export type SqliteColumn = { name: string; type: string; id?: boolean };

const affinity: Record<string, string> = {
  int: "INTEGER",
  uuid: "TEXT",
  string: "TEXT",
  bool: "INTEGER",
  decimal: "NUMERIC",
  float: "REAL",
  datetime: "TEXT",
  binary: "BLOB",
};

export const SqliteStatementBuilder = {
  quote: (id: string): string => `"${id.replaceAll('"', '""')}"`,

  quoteQualified: (id: string): string =>
    id.split(".").map(SqliteStatementBuilder.quote).join("."),

  columnType: (canonical: string): string =>
    affinity[canonical] ??
    (() => {
      throw new DatasourceError(`Unknown sqlite type "${canonical}"`);
    })(),

  createTable: (table: string, columns: SqliteColumn[]) =>
    StatementBuilder.createTable(
      SqliteStatementBuilder.quote(table),
      columns.map((column) => ({
        name: SqliteStatementBuilder.quote(column.name),
        type: column.id
          ? `${SqliteStatementBuilder.columnType(column.type)} PRIMARY KEY`
          : SqliteStatementBuilder.columnType(column.type),
      })),
    ),

  createQuery: (
    table: string,
    fields: string[],
    where?: IWhere,
    joins: Join[] = [],
  ) =>
    StatementBuilder.createQuery(
      SqliteStatementBuilder.quote(table),
      fields.map(SqliteStatementBuilder.quoteQualified),
      where,
      joins.map((join) => ({
        kind: join.kind,
        table: SqliteStatementBuilder.quoteQualified(join.table),
        left: SqliteStatementBuilder.quoteQualified(join.left),
        right: SqliteStatementBuilder.quoteQualified(join.right),
      })),
    ),

  createInsert: (table: string, fields: Field[]) =>
    StatementBuilder.createInsert(SqliteStatementBuilder.quote(table), fields),

  createUpdate: (table: string, fields: Field[], where?: IWhere) =>
    StatementBuilder.createUpdate(SqliteStatementBuilder.quote(table), fields, where),

  createDelete: (table: string, where?: IWhere) =>
    StatementBuilder.createDelete(SqliteStatementBuilder.quote(table), where),
};
