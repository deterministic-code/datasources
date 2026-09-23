import type { IDeleteStatement } from "./statements/IDeleteStatement.ts";
import type { IInsertStatement } from "./statements/IInsertStatement.ts";
import type { IReturnStatement } from "./statements/IReturnStatement.ts";
import type { IUpdateStatement } from "./statements/IUpdateStatement.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { Cells } from "./converters/Cell.ts";

export type CRUDRepositoryOptions<T = Cells> = ReadonlyRepositoryOptions<T> & {
  ids?: string[];
  insert?: IMapper<IInsertStatement, IInsertStatement>;
  update?: IMapper<IUpdateStatement, IUpdateStatement>;
  delete?: IMapper<IDeleteStatement, IDeleteStatement>;
  return?: IMapper<IReturnStatement, IReturnStatement>;
  create?: IMapper<Scalar, T>;
};
