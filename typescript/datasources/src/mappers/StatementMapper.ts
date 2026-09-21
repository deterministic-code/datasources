import type { IDeleteStatement } from "../statements/IDeleteStatement.ts";
import type { IInsertStatement } from "../statements/IInsertStatement.ts";
import type { IReturnStatement } from "../statements/IReturnStatement.ts";
import type { IExecuteStoredProcedureStatement } from "../statements/IExecuteStoredProcedureStatement.ts";
import type { IUpdateStatement } from "../statements/IUpdateStatement.ts";
import type { IQuery } from "../statements/IQuery.ts";
import { TransformMapper } from "./TransformMapper.ts";
import type { IMapper } from "./IMapper.ts";

export type StatementMapperOptions<T> = {
  entity?: IMapper<string, string>;
  columns?: IMapper<string, string>;
  rows?: IMapper<T, T>;
  query?: IMapper<IQuery, IQuery>;
  insert?: IMapper<IInsertStatement, IInsertStatement>;
  update?: IMapper<IUpdateStatement, IUpdateStatement>;
  delete?: IMapper<IDeleteStatement, IDeleteStatement>;
  storedProcedure?: IMapper<IExecuteStoredProcedureStatement, IExecuteStoredProcedureStatement>;
  return?: IMapper<IReturnStatement, IReturnStatement>;
};

export class StatementMapper<T> implements IMapper<T, T> {
  readonly entity: IMapper<string, string>;
  readonly columns: IMapper<string, string>;
  readonly rows: IMapper<T, T>;
  readonly query?: IMapper<IQuery, IQuery>;
  readonly insert?: IMapper<IInsertStatement, IInsertStatement>;
  readonly update?: IMapper<IUpdateStatement, IUpdateStatement>;
  readonly delete?: IMapper<IDeleteStatement, IDeleteStatement>;
  readonly storedProcedure?: IMapper<IExecuteStoredProcedureStatement, IExecuteStoredProcedureStatement>;
  readonly return?: IMapper<IReturnStatement, IReturnStatement>;

  constructor(options: StatementMapperOptions<T> = {}) {
    this.entity = options.entity ?? new TransformMapper((name) => name, (name) => name);
    this.columns = options.columns ?? new TransformMapper((name) => name, (name) => name);
    this.rows =
      options.rows ?? new TransformMapper<T, T>((value) => value, (value) => value);
    this.query = options.query;
    this.insert = options.insert;
    this.update = options.update;
    this.delete = options.delete;
    this.storedProcedure = options.storedProcedure;
    this.return = options.return;
  }

  map = (value: T): T => this.rows.map(value);
  reverseMap = (value: T): T => this.rows.reverseMap(value);
}
