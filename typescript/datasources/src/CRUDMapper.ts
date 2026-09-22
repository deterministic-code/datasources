import type { IDeleteStatement } from "./statements/IDeleteStatement.ts";
import type { IInsertStatement } from "./statements/IInsertStatement.ts";
import type { IReturnStatement } from "./statements/IReturnStatement.ts";
import type { IUpdateStatement } from "./statements/IUpdateStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { TransformMapper } from "./mappers/TransformMapper.ts";
import type { IMapper } from "./mappers/IMapper.ts";

export type CRUDMapperOptions<T> = {
  entity?: IMapper<string, string>;
  columns?: IMapper<string, string>;
  rows?: IMapper<T, T>;
  query?: IMapper<IQuery, IQuery>;
  insert?: IMapper<IInsertStatement, IInsertStatement>;
  update?: IMapper<IUpdateStatement, IUpdateStatement>;
  delete?: IMapper<IDeleteStatement, IDeleteStatement>;
  return?: IMapper<IReturnStatement, IReturnStatement>;
  create?: IMapper<Scalar, T>;
};

const defaultCreate = <T>(): IMapper<Scalar, T> =>
  new TransformMapper(
    (value) => {
      if (value == null) return {} as T;
      return { id: value } as T;
    },
    () => null,
  );

export class CRUDMapper<T> {
  readonly entity: IMapper<string, string>;
  readonly columns: IMapper<string, string>;
  readonly rows: IMapper<T, T>;
  readonly query?: IMapper<IQuery, IQuery>;
  readonly insert?: IMapper<IInsertStatement, IInsertStatement>;
  readonly update?: IMapper<IUpdateStatement, IUpdateStatement>;
  readonly delete?: IMapper<IDeleteStatement, IDeleteStatement>;
  readonly return?: IMapper<IReturnStatement, IReturnStatement>;
  readonly create: IMapper<Scalar, T>;

  constructor(options: CRUDMapperOptions<T> = {}) {
    this.entity = options.entity ?? new TransformMapper((name) => name, (name) => name);
    this.columns = options.columns ?? new TransformMapper((name) => name, (name) => name);
    this.rows =
      options.rows ?? new TransformMapper<T, T>((value) => value, (value) => value);
    this.query = options.query;
    this.insert = options.insert;
    this.update = options.update;
    this.delete = options.delete;
    this.return = options.return;
    this.create = options.create ?? defaultCreate();
  }
}
