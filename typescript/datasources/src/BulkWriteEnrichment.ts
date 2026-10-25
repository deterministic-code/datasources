import type { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import { enrichmentsOf } from "./ReadonlyRepository.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import type { WriteMode } from "./WriteMode.ts";

export const afterBulkWrite = async <T extends object>(
  repo: CRUDRepository<T>,
  options: CRUDRepositoryOptions<T>,
  items: T[],
  mode: WriteMode,
): Promise<void> => {
  if (items.length === 0 || !enrichmentsOf(repo)?.length) return;
  const write = options.writeEnrichment ?? new DefaultWriteEnrichmentStrategy<T>();
  const tree = new RepositoryTree(new RepositoryTreeNode(repo, ""));
  await (write.persistBulk ?? write.persist)(tree, items, mode);
};
