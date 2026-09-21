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
    new CreateTableStatement(
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
    new QueryStatement(
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
    new InsertStatement(SqliteStatementBuilder.quote(table), fields),

  createStoredProcedure: (name: string, parameters: Field[] = []) =>
    new StoredProcedureStatement(SqliteStatementBuilder.quote(name), parameters),

  createUpdate: (table: string, fields: Field[], where?: IWhere) =>
    new UpdateStatement(SqliteStatementBuilder.quote(table), fields, where),

  createDelete: (table: string, where?: IWhere) =>
    new DeleteStatement(SqliteStatementBuilder.quote(table), where),
};
