import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const get = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
  (item as never as Props)[name];

const present = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

const linkWhere = (
  association: NonNullable<Enrichment["association"]>,
  parentKey: Scalar,
  relatedKey: Scalar,
): Where =>
  new Where(
    new Operator.And(
      new Operator.Equals(association.parent, parentKey),
      new Operator.Equals(association.related, relatedKey),
    ),
  );

export class CascadeDeleteEnrichmentStrategy<T> extends DeleteEnrichmentStrategy<T> {
  async deleteChildren<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: T,
    deleteNested: (rows: TChild[]) => Promise<void>,
  ): Promise<void> {
    const existing = await DeleteEnrichmentStrategy.load(writer, enrichment, parent);
    await deleteNested(existing);
    for (const row of existing) await writer.delete(row, DeleteEnrichmentStrategy.childrenWhere(writer, enrichment, parent));
  }

  async deleteLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const key = get(parent, enrichment.sourceProperty);
    if (!present(key)) return;
    for (const row of await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, key)) {
      const id = get(row, association.related);
      if (!present(id)) continue;
      await writer.delete(row, linkWhere(association, key, id));
    }
  }
}
