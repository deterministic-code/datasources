import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export interface IRepositoryTreeNode {
  readonly value: object;
  readonly propertyName: string;
  readonly parent?: IRepositoryTreeNode;
  readonly items: IRepositoryTreeNode[];
  readonly enrichment?: Enrichment;
  include<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    select?: string,
    many?: boolean,
  ): IRepositoryTreeNode;
  includeLinked<TRelated extends object, TLink extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    link: ICRUDRepository<TLink>,
    parent: string,
    related: string,
  ): IRepositoryTreeNode;
  enrichFrom(parents: object[]): Promise<void>;
}
