import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

const get = (item: object, name: string): Scalar | object | undefined => props(item)[name];

const present = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

const writerOf = (entry: StoredEnrichment, into: string): ICRUDRepository<object> => {
  if (entry.persist) return entry.persist;
  throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
};

const linkWriter = (entry: StoredEnrichment, into: string): ICRUDRepository<object> => {
  if (entry.link) return entry.link;
  throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
};

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
  async deleteOwned(
    entry: StoredEnrichment,
    enrichment: Enrichment,
    parent: object,
    deleteNested: (rows: object[]) => Promise<void>,
  ): Promise<void> {
    const writer = writerOf(entry, enrichment.into);
    const existing = await DeleteEnrichmentStrategy.load(writer, enrichment, parent);
    await deleteNested(existing);
    for (const row of existing) await writer.delete(row);
  }

  async deleteLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const writer = linkWriter(entry, enrichment.into);
    const key = get(parent, enrichment.from);
    if (!present(key)) return;
    for (const row of await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, key)) {
      const id = get(row, association.related);
      if (!present(id)) continue;
      await writer.delete(row, linkWhere(association, key, id));
    }
  }
}
