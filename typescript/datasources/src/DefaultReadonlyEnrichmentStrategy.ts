import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { keyOf, props, scalar, sourceKey } from "./RepositoryTreeNode.ts";
import { Operator, Where } from "./statements/index.ts";

export class DefaultReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  apply<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    return enrichment.many
      ? this.applyMany(via, enrichment, parents, sources)
      : this.applyLookup(via, enrichment, parents, sources);
  }

  async applyLinked<TRelated extends object, TLink extends object>(
    via: IReadonlyRepository<TRelated>,
    link: ICRUDRepository<TLink>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const association = enrichment.association ?? (() => {
      throw new DatasourceError("Missing association");
    })();
    for (const [index, parent] of parents.entries()) {
      const key = sourceKey(parent, enrichment.from, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for await (const row of link.read(
        new Where(new Operator.Equals(association.parent, scalar(key))),
      )) {
        const id = props(row)[association.related];
        if (id === undefined || id === null || typeof id === "object") continue;
        const entity = await via.readSingle(new Where(new Operator.Equals(enrichment.by, id)));
        if (entity) related.push(entity);
      }
      Object.assign(parent, { [enrichment.into]: related });
    }
  }

  protected async applyLookup<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const cache = new Map<string, TRelated | undefined>();
    for (const [index, parent] of parents.entries()) {
      const key = sourceKey(parent, enrichment.from, sources, index);
      if (key === undefined || key === null) continue;
      const cacheKey = keyOf(key);
      if (!cache.has(cacheKey))
        cache.set(
          cacheKey,
          await via.readSingle(new Where(new Operator.Equals(enrichment.by, scalar(key)))),
        );
      const related = cache.get(cacheKey);
      if (!related) continue;
      Object.assign(parent, {
        [enrichment.into]: enrichment.select ? props(related)[enrichment.select] : related,
      });
    }
  }

  protected async applyMany<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: object[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    for (const [index, parent] of parents.entries()) {
      const key = sourceKey(parent, enrichment.from, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for await (const item of via.read(
        new Where(new Operator.Equals(enrichment.by, scalar(key))),
      ))
        related.push(item);
      Object.assign(parent, { [enrichment.into]: related });
    }
  }
}
