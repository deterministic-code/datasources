import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, WhereClause } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

const get = (item: object, name: string): Scalar | object | undefined => props(item)[name];

const present = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

const isWriter = (via: object): via is ICRUDRepository<object> =>
  "create" in via && typeof via.create === "function";

const writerOf = (via: object, into: string): ICRUDRepository<object> =>
  isWriter(via)
    ? via
    : (() => {
        throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
      })();

const linkWriter = (child: IRepositoryTreeNode, into: string): ICRUDRepository<object> => {
  const link = (child as RepositoryTreeNode).link;
  if (!link) throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
  return link;
};

const linkWhere = (
  association: NonNullable<Enrichment["association"]>,
  parentKey: Scalar,
  relatedKey: Scalar,
): WhereClause =>
  new WhereClause(
    new Operator.And(
      new Operator.Equals(association.parent, parentKey),
      new Operator.Equals(association.related, relatedKey),
    ),
  );

export class CascadeDeleteEnrichmentStrategy<T> extends DeleteEnrichmentStrategy<T> {
  async deleteOwned(
    node: IRepositoryTreeNode,
    enrichment: Enrichment,
    parent: object,
    deleteNested: (rows: object[]) => Promise<void>,
  ): Promise<void> {
    const writer = writerOf(node.value, enrichment.into);
    const existing = await DeleteEnrichmentStrategy.load(writer, enrichment, parent);
    await deleteNested(existing);
    for (const row of existing) await writer.delete(row);
  }

  async deleteLinked(node: IRepositoryTreeNode, enrichment: Enrichment, parent: object): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const writer = linkWriter(node, enrichment.into);
    const key = get(parent, enrichment.from);
    if (!present(key)) return;
    for (const row of await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, key)) {
      const id = get(row, association.related);
      if (!present(id)) continue;
      await writer.delete(row, linkWhere(association, key, id));
    }
  }
}
