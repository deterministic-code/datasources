import type { Enrichment } from "./Enrichment.ts";

export interface IRepositoryTreeNode {
  readonly value: object;
  readonly propertyName: string;
  readonly parent?: IRepositoryTreeNode;
  readonly enrichment?: Enrichment;
}
