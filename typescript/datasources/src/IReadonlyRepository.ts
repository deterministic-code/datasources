import type { Parameter } from "./Parameter.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IRepository } from "./IRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";

export type ReadQuery = ISelectStatement | IWhere | IExecuteStoredProcedureStatement;

export interface IReadonlyRepository<T = unknown> extends IRepository<T> {
  readSingle(
    query?: ReadQuery,
    parameters?: Parameter[],
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): Promise<T | undefined>;
  read(
    query?: ReadQuery,
    parameters?: Parameter[],
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): AsyncIterable<T>;
}
