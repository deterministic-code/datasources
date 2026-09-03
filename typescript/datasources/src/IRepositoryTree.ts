import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";

export interface IRepositoryTree {
  readonly parent: IRepositoryTreeNode;
}
