import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";

export class LoadLevelOrderReadonlyEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T> {
  constructor(batchSize = 500) {
    super();
    if (batchSize < 1) throw new DatasourceError("batchSize must be at least 1");
    this.batchSize = batchSize;
  }

  withBatchSize = (batchSize: number): LoadLevelOrderReadonlyEnrichmentStrategy<T> =>
    new LoadLevelOrderReadonlyEnrichmentStrategy<T>(batchSize);

  override async applyLinked<TRelated extends object, TLink extends object>(
    via: IReadonlyRepository<TRelated>,
    link: IReadonlyRepository<TLink>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const association = enrichment.association ?? (() => {
      throw new DatasourceError("Missing association");
    })();
    const links = await this.loadByKeys(link, association.parent, this.distinctKeys(parents, sources, enrichment));
    const relatedIds: Scalar[] = [];
    const seen = new Set<string>();
    for (const row of links) {
      const id = this.props(row)[association.related];
      if (id === undefined || id === null) continue;
      const cacheKey = this.keyOf(id);
      if (seen.has(cacheKey)) continue;
      seen.add(cacheKey);
      relatedIds.push(this.scalar(id));
    }
    const relatedById = new Map<string, TRelated>();
    for (const item of await this.loadByKeys(via, enrichment.relatedKey, relatedIds)) {
      const id = this.props(item)[enrichment.relatedKey];
      if (id === undefined || id === null) continue;
      const cacheKey = this.keyOf(id);
      if (!relatedById.has(cacheKey)) relatedById.set(cacheKey, item);
    }
    const linksByParent = new Map<string, TLink[]>();
    for (const row of links) {
      const parentKey = this.props(row)[association.parent];
      if (parentKey === undefined || parentKey === null) continue;
      const cacheKey = this.keyOf(parentKey);
      const list = linksByParent.get(cacheKey) ?? [];
      list.push(row);
      linksByParent.set(cacheKey, list);
    }
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for (const row of linksByParent.get(this.keyOf(key)) ?? []) {
        const id = this.props(row)[association.related];
        if (id === undefined || id === null) continue;
        const entity = relatedById.get(this.keyOf(id));
        if (entity) related.push(entity);
      }
      Object.assign(parent, { [enrichment.destinationProperty]: related });
    }
  }
}
