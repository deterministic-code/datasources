import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IRepository } from "./IRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";

export interface IReadonlyRepository<T = unknown> extends IRepository<T> {
  readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>;
  read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>;
}
