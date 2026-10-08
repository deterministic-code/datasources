import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import { EnrichmentPropertyType } from "./EnrichmentPropertyType.ts";
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
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const type = enrichment.type ?? EnrichmentPropertyType.Object;
    const run = {
      [EnrichmentPropertyType.List]: () => this.applyList(via, enrichment, parents, sources),
      [EnrichmentPropertyType.Object]: () => this.applyObject(via, enrichment, parents, sources),
      [EnrichmentPropertyType.Dictionary]: () => this.applyDictionary(via, enrichment, parents, sources),
    }[type];
    return run ? run() : Promise.reject(new DatasourceError(`Unsupported enrichment type "${type}"`));
  }

  async applyLinked<TRelated extends object, TLink extends object>(
    via: IReadonlyRepository<TRelated>,
    link: IReadonlyRepository<TLink>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const association = enrichment.association ?? (() => {
      throw new DatasourceError("Missing association");
    })();
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment.sourceProperty, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for await (const row of link.read(
        new Where(new Operator.Equals(association.parent, this.scalar(key))),
      )) {
        const id = this.props(row)[association.related];
        if (id === undefined || id === null || typeof id === "object") continue;
        const entity = await via.readSingle(new Where(new Operator.Equals(enrichment.relatedKey, id)));
        if (entity) related.push(entity);
      }
      Object.assign(parent, { [enrichment.destinationProperty]: related });
    }
  }

  protected async applyObject<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const cache = new Map<string, TRelated | undefined>();
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment.sourceProperty, sources, index);
      if (key === undefined || key === null) continue;
      const cacheKey = this.keyOf(key);
      if (!cache.has(cacheKey))
        cache.set(
          cacheKey,
          await via.readSingle(new Where(new Operator.Equals(enrichment.relatedKey, this.scalar(key)))),
        );
      const related = cache.get(cacheKey);
      if (!related) continue;
      Object.assign(parent, {
        [enrichment.destinationProperty]: enrichment.relatedProperty ? this.props(related)[enrichment.relatedProperty] : related,
      });
    }
  }

  protected async applyList<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment.sourceProperty, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for await (const item of via.read(
        new Where(new Operator.Equals(enrichment.relatedKey, this.scalar(key))),
      ))
        related.push(item);
      Object.assign(parent, { [enrichment.destinationProperty]: related });
    }
  }

  protected async applyDictionary<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const mapKey = mapKeyName(enrichment.relatedKey);
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment.sourceProperty, sources, index);
      if (key === undefined || key === null) continue;
      const map: Record<string, Scalar | object> = {};
      for await (const item of via.read(
        new Where(new Operator.Equals(enrichment.relatedKey, this.scalar(key))),
      )) {
        const row = this.props(item);
        const entry = row[mapKey];
        if (entry === undefined || entry === null || typeof entry === "object") continue;
        map[String(entry)] = enrichment.relatedProperty
          ? (row[enrichment.relatedProperty] ?? null)
          : Object.fromEntries(
              Object.entries(row).filter(
                ([name]) => name !== mapKey && name !== enrichment.relatedKey && name !== "Id" && name !== "id",
              ),
            );
      }
      Object.assign(parent, { [enrichment.destinationProperty]: map });
    }
  }

  protected sourceKey(
    parent: object,
    name: string,
    sources: (Row | undefined)[] | undefined,
    index: number,
  ): Scalar | object | undefined {
    const cell = sources?.[index]?.[name];
    const raw = cell !== undefined && cell !== null ? cell : this.props(parent)[name];
    return this.lookupKey(raw);
  }

  protected props = (item: object): Props => item as Props;

  protected keyOf = (key: Scalar | object): string => String(this.lookupKey(key));

  protected lookupKey = (value: Scalar | object | undefined): Scalar | object | undefined => {
    if (value === undefined || value === null || typeof value !== "object") return value;
    const id = this.props(value).Id ?? this.props(value).id;
    return id !== undefined && id !== null && typeof id !== "object" ? id : value;
  };

  protected scalar = (value: Scalar | object): Scalar => {
    const key = this.lookupKey(value);
    return key === null ||
      typeof key === "string" ||
      typeof key === "number" ||
      typeof key === "boolean" ||
      typeof key === "bigint"
      ? key
      : (() => {
          throw new DatasourceError("Enrichment key must be a scalar");
        })();
  };
}

const mapKeyName = (relatedKey: string): string =>
  relatedKey.includes("_") || relatedKey[0] !== relatedKey[0]?.toUpperCase() ? "key" : "Key";

