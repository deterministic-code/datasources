import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";

export class RepositoryTree implements IRepositoryTree {
  constructor(readonly parent: IRepositoryTreeNode) {}
}
