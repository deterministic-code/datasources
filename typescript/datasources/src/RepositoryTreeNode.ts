import type { Enrichment } from "./Enrichment.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";

export class RepositoryTreeNode implements IRepositoryTreeNode {
  constructor(
    readonly value: object,
    readonly propertyName: string,
    readonly parent?: IRepositoryTreeNode,
    readonly enrichment?: Enrichment,
  ) {}
}
