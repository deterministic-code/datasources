import type { IDeleteStatement } from "./statements/IDeleteStatement.ts";
import type { IInsertStatement } from "./statements/IInsertStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IUpdateStatement } from "./statements/IUpdateStatement.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { QueryStatement } from "./statements/QueryStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import type { Field } from "./builders/Field.ts";
import type { IMapper } from "./mappers/IMapper.ts";

export type RepositoryStatements = {
  query: (where?: IWhere) => IQuery;
  insert: (fields: Field[]) => IInsertStatement;
  update: (fields: Field[], where?: IWhere) => IUpdateStatement;
  delete: (where?: IWhere) => IDeleteStatement;
};

const mapped = <T>(built: T, mapper?: IMapper<T, T>): T =>
  mapper?.map(built) ?? built;

export const statementsOf = (
  table: string,
  columns: string[],
  query?: IMapper<IQuery, IQuery>,
  insert?: IMapper<IInsertStatement, IInsertStatement>,
  update?: IMapper<IUpdateStatement, IUpdateStatement>,
  deleteMapper?: IMapper<IDeleteStatement, IDeleteStatement>,
): RepositoryStatements => ({
  query: (where) => mapped(new QueryStatement({ table, fields: columns, where }), query),
  insert: (fields) => mapped(new InsertStatement(table, fields), insert),
  update: (fields, where) => mapped(new UpdateStatement(table, fields, where), update),
  delete: (where) => mapped(new DeleteStatement(table, where), deleteMapper),
});
