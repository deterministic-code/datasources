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
    new CreateTableStatement(
      PostgresStatementBuilder.quote(table),
      columns.map((column) => ({
        name: PostgresStatementBuilder.quote(column.name),
        type: column.id
          ? `${PostgresStatementBuilder.columnType(column.type)} PRIMARY KEY`
          : PostgresStatementBuilder.columnType(column.type),
      })),
    ),

  createQuery: (table: string, fields: string[], where?: IWhere) =>
    new QueryStatement(
      PostgresStatementBuilder.quote(table),
      fields.map(PostgresStatementBuilder.quote),
      where,
    ),

  createInsert: (table: string, fields: Field[]) =>
    new InsertStatement(PostgresStatementBuilder.quote(table), fields),

  createStoredProcedure: (name: string, parameters: Field[] = []) =>
    new StoredProcedureStatement(PostgresStatementBuilder.quote(name), parameters),

  createUpdate: (table: string, fields: Field[], where?: IWhere) =>
    new UpdateStatement(PostgresStatementBuilder.quote(table), fields, where),

  createDelete: (table: string, where?: IWhere) =>
    new DeleteStatement(PostgresStatementBuilder.quote(table), where),
};
