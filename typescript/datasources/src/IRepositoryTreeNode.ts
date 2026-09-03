import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export interface IRepositoryTreeNode {
  readonly value: object;
  readonly propertyName: string;
  readonly parent?: IRepositoryTreeNode;
  readonly items: IRepositoryTreeNode[];
  include<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    select?: string,
    many?: boolean,
  ): this;
  enrichFrom(parents: object[]): Promise<void>;
}
