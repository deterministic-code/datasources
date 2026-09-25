import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IReadonlyEnrichmentBinder } from "./IReadonlyEnrichmentBinder.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { Row } from "./Row.ts";

export interface IReadonlyEnrichmentStrategy<T extends object>
  extends IReadonlyEnrichmentBinder {
  read(
    inner: IReadonlyRepository<T>,
    tree: IRepositoryTree,
    query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<T>;
  enrich(tree: IRepositoryTree, items: T[], sources?: (Row | undefined)[]): Promise<void>;
}
