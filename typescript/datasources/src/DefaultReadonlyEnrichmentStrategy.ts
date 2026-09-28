import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

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
      const key = this.sourceKey(parent, enrichment.from, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for await (const row of link.read(
        new Where(new Operator.Equals(association.parent, this.scalar(key))),
      )) {
        const id = this.props(row)[association.related];
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
      const key = this.sourceKey(parent, enrichment.from, sources, index);
      if (key === undefined || key === null) continue;
      const cacheKey = this.keyOf(key);
      if (!cache.has(cacheKey))
        cache.set(
          cacheKey,
          await via.readSingle(new Where(new Operator.Equals(enrichment.by, this.scalar(key)))),
        );
      const related = cache.get(cacheKey);
      if (!related) continue;
      Object.assign(parent, {
        [enrichment.into]: enrichment.select ? this.props(related)[enrichment.select] : related,
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
      const key = this.sourceKey(parent, enrichment.from, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for await (const item of via.read(
        new Where(new Operator.Equals(enrichment.by, this.scalar(key))),
      ))
        related.push(item);
      Object.assign(parent, { [enrichment.into]: related });
    }
  }

  protected sourceKey(
    parent: object,
    name: string,
    sources: (Row | undefined)[] | undefined,
    index: number,
  ): Scalar | object | undefined {
    const cell = sources?.[index]?.[name];
    if (cell !== undefined && cell !== null) return cell;
    return this.props(parent)[name];
  }

  protected props = (item: object): Props => item as Props;

  protected keyOf = (key: Scalar | object): string => String(key);

  protected scalar = (value: Scalar | object): Scalar =>
    value === null ||
    typeof value === "string" ||
    typeof value === "number" ||
    typeof value === "boolean" ||
    typeof value === "bigint"
      ? value
      : (() => {
          throw new DatasourceError("Enrichment key must be a scalar");
        })();
}
