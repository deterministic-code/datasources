import type { IDeleteStatement } from "./builders/IDeleteStatement.ts";
import type { IInsertStatement } from "./builders/IInsertStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IUpdateStatement } from "./builders/IUpdateStatement.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { StatementBuilder } from "./builders/index.ts";
import type { Field } from "./builders/Field.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { StatementMapper } from "./mappers/StatementMapper.ts";

export type RepositoryStatements = {
  query: (where?: IWhere) => IQuery;
  insert: (fields: Field[]) => IInsertStatement;
  update: (fields: Field[], where?: IWhere) => IUpdateStatement;
  delete: (where?: IWhere) => IDeleteStatement;
};

const mapped = <T>(built: T, mapper?: IMapper<T, T>): T =>
  mapper?.map(built) ?? built;

export const statementsOf = <T>(
  table: string,
  columns: string[],
  mapper?: StatementMapper<T>,
): RepositoryStatements => ({
  query: (where) => mapped(StatementBuilder.createQuery(table, columns, where), mapper?.query),
  insert: (fields) => mapped(StatementBuilder.createInsert(table, fields), mapper?.insert),
  update: (fields, where) =>
    mapped(StatementBuilder.createUpdate(table, fields, where), mapper?.update),
  delete: (where) => mapped(StatementBuilder.createDelete(table, where), mapper?.delete),
});
