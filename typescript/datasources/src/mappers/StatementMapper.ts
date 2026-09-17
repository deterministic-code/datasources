import type { IDeleteStatement } from "../builders/IDeleteStatement.ts";
import type { IInsertStatement } from "../builders/IInsertStatement.ts";
import type { IStoredProcedureStatement } from "../builders/IStoredProcedureStatement.ts";
import type { IUpdateStatement } from "../builders/IUpdateStatement.ts";
import type { IQuery } from "../IQuery.ts";
import { NameSourceDestinationMapper } from "./NameSourceDestinationMapper.ts";
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
  storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>;
};

export class StatementMapper<T> implements IMapper<T, T> {
  readonly entity: IMapper<string, string>;
  readonly columns: IMapper<string, string>;
  readonly rows: IMapper<T, T>;
  readonly query?: IMapper<IQuery, IQuery>;
  readonly insert?: IMapper<IInsertStatement, IInsertStatement>;
  readonly update?: IMapper<IUpdateStatement, IUpdateStatement>;
  readonly delete?: IMapper<IDeleteStatement, IDeleteStatement>;
  readonly storedProcedure?: IMapper<IStoredProcedureStatement, IStoredProcedureStatement>;

  constructor(options: StatementMapperOptions<T> = {}) {
    this.entity = options.entity ?? new NameSourceDestinationMapper("", "");
    this.columns = options.columns ?? new NameSourceDestinationMapper("", "");
    this.rows =
      options.rows ?? new TransformMapper<T, T>((value) => value, (value) => value);
    this.query = options.query;
    this.insert = options.insert;
    this.update = options.update;
    this.delete = options.delete;
    this.storedProcedure = options.storedProcedure;
  }

  map = (value: T): T => this.rows.map(value);
  reverseMap = (value: T): T => this.rows.reverseMap(value);
}
