import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";

export interface IEnriched<T> {
  addEnriched<TRelated extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    select?: keyof TRelated & string,
    many?: boolean,
    nest?: (node: IRepositoryTreeNode) => void,
  ): this;
  enrich(items: T[]): Promise<T[]>;
}
