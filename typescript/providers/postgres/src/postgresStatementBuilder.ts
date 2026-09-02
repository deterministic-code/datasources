import {
  DatasourceError,
  StatementBuilder,
  type Field,
  type IWhere,
} from "@datasources/core";

export type PostgresColumn = { name: string; type: string; id?: boolean };

const affinity: Record<string, string> = {
  int: "INTEGER",
  uuid: "UUID",
  string: "TEXT",
  bool: "BOOLEAN",
  decimal: "NUMERIC",
  float: "DOUBLE PRECISION",
  datetime: "TIMESTAMPTZ",
  binary: "BYTEA",
};

export const PostgresStatementBuilder = {
  quote: (id: string): string => `"${id.replaceAll('"', '""')}"`,

  columnType: (canonical: string): string =>
    affinity[canonical] ??
    (() => {
      throw new DatasourceError(`Unknown postgres type "${canonical}"`);
    })(),

  createTable: (table: string, columns: PostgresColumn[]) =>
    StatementBuilder.createTable(
      PostgresStatementBuilder.quote(table),
      columns.map((column) => ({
        name: PostgresStatementBuilder.quote(column.name),
        type: column.id
          ? `${PostgresStatementBuilder.columnType(column.type)} PRIMARY KEY`
          : PostgresStatementBuilder.columnType(column.type),
      })),
    ),

  createQuery: (table: string, fields: string[], where?: IWhere) =>
    StatementBuilder.createQuery(
      PostgresStatementBuilder.quote(table),
      fields.map(PostgresStatementBuilder.quote),
      where,
    ),

  createInsert: (table: string, fields: Field[]) =>
    StatementBuilder.createInsert(PostgresStatementBuilder.quote(table), fields),

  createUpdate: (table: string, fields: Field[], where?: IWhere) =>
    StatementBuilder.createUpdate(PostgresStatementBuilder.quote(table), fields, where),

  createDelete: (table: string, where?: IWhere) =>
    StatementBuilder.createDelete(PostgresStatementBuilder.quote(table), where),
};
