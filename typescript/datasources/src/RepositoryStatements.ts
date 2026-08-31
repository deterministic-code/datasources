import type { IDeleteStatement } from "./builders/IDeleteStatement.ts";
import type { IInsertStatement } from "./builders/IInsertStatement.ts";
import type { IQuery } from "./IQuery.ts";
import type { IUpdateStatement } from "./builders/IUpdateStatement.ts";
import type { IWhere } from "./IWhere.ts";
import { StatementBuilder } from "./builders/index.ts";
import type { Field } from "./builders/Field.ts";

export type RepositoryStatements = {
  query: (where?: IWhere) => IQuery;
  insert: (fields: Field[]) => IInsertStatement;
  update: (fields: Field[], where?: IWhere) => IUpdateStatement;
  delete: (where?: IWhere) => IDeleteStatement;
};

export const statementsOf = (table: string, columns: string[]): RepositoryStatements => ({
  query: (where) => StatementBuilder.createQuery(table, columns, where),
  insert: (fields) => StatementBuilder.createInsert(table, fields),
  update: (fields, where) => StatementBuilder.createUpdate(table, fields, where),
  delete: (where) => StatementBuilder.createDelete(table, where),
});
