import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IRepository } from "./IRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";

export interface IReadonlyRepository<T = unknown> extends IRepository<T> {
  readSingle(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined>;
  read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T>;
}
