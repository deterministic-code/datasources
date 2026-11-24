import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { dictionaryMapKey, dictionaryValueColumns, type Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { Schema } from "./Schema.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

export class LoadLevelOrderReadonlyEnrichmentStrategy<T extends object> extends DefaultReadonlyEnrichmentStrategy<T> {
  private readonly batchSize: number;

  constructor(batchSize = 500) {
    super();
    if (batchSize < 1) throw new DatasourceError("batchSize must be at least 1");
    this.batchSize = batchSize;
  }

  withBatchSize = (batchSize: number): LoadLevelOrderReadonlyEnrichmentStrategy<T> =>
    new LoadLevelOrderReadonlyEnrichmentStrategy<T>(batchSize);

  protected override async applyObject<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const index = new Map<string, TRelated>();
    for (const item of await this.loadByKeys(via, enrichment.relatedKey, this.distinctKeys(parents, sources, enrichment))) {
      const cacheKey = this.rowMatchKey(item, enrichment);
      if (cacheKey !== undefined && !index.has(cacheKey)) index.set(cacheKey, item);
    }
    for (const [indexOfParent, parent] of parents.entries()) {
      const cacheKey = this.matchKey(parent, enrichment, sources, indexOfParent);
      if (cacheKey === undefined) continue;
      const related = index.get(cacheKey);
      if (!related) continue;
      Object.assign(parent, {
        [enrichment.destinationProperty]: enrichment.relatedProperty ? this.props(related)[enrichment.relatedProperty] : related,
      });
    }
  }

  protected override async applyList<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const index = this.group(
      await this.loadByKeys(via, enrichment.relatedKey, this.distinctKeys(parents, sources, enrichment)),
      enrichment,
    );
    for (const [indexOfParent, parent] of parents.entries()) {
      const cacheKey = this.matchKey(parent, enrichment, sources, indexOfParent);
      if (cacheKey === undefined) continue;
      Object.assign(parent, { [enrichment.destinationProperty]: index.get(cacheKey) ?? [] });
    }
  }

  protected override async applyDictionary<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const index = this.group(
      await this.loadByKeys(via, enrichment.relatedKey, this.distinctKeys(parents, sources, enrichment)),
      enrichment,
    );
    const schema = via.schema;
    const mapKey = dictionaryMapKey(enrichment);
    for (const [indexOfParent, parent] of parents.entries()) {
      const cacheKey = this.matchKey(parent, enrichment, sources, indexOfParent);
      if (cacheKey === undefined) continue;
      const map: Record<string, Scalar | object> = {};
      for (const item of index.get(cacheKey) ?? []) {
        const row = this.props(item);
        const entry = row[mapKey];
        if (entry === undefined || entry === null) continue;
        map[String(entry)] = enrichment.relatedProperty
          ? (row[enrichment.relatedProperty] ?? null)
          : this.valueOf(row, schema, enrichment);
      }
      Object.assign(parent, { [enrichment.destinationProperty]: map });
    }
  }

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

  private async loadByKeys<TItem extends object>(
    via: IReadonlyRepository<TItem>,
    column: string,
    keys: Scalar[],
  ): Promise<TItem[]> {
    const related: TItem[] = [];
    if (keys.length === 0) return related;
    for (const chunk of this.chunks(keys)) {
      for await (const item of via.read(new Where(new Operator.In(column, chunk))))
        related.push(item);
    }
    return related;
  }

  private distinctKeys(parents: T[], sources: (Row | undefined)[] | undefined, enrichment: Enrichment): Scalar[] {
    const keys: Scalar[] = [];
    const seen = new Set<string>();
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment, sources, index);
      if (key === undefined || key === null) continue;
      const cacheKey = this.keyOf(key);
      if (seen.has(cacheKey)) continue;
      seen.add(cacheKey);
      keys.push(this.scalar(key));
    }
    return keys;
  }

  private group<TRelated>(items: TRelated[], enrichment: Enrichment): Map<string, TRelated[]> {
    const index = new Map<string, TRelated[]>();
    for (const item of items) {
      const cacheKey = this.rowMatchKey(item, enrichment);
      if (cacheKey === undefined) continue;
      const list = index.get(cacheKey) ?? [];
      list.push(item);
      index.set(cacheKey, list);
    }
    return index;
  }

  private chunks = <TItem>(items: TItem[]): TItem[][] => {
    const out: TItem[][] = [];
    for (let offset = 0; offset < items.length; offset += this.batchSize)
      out.push(items.slice(offset, offset + this.batchSize));
    return out;
  };

  private valueOf = (row: Props, schema: Schema, enrichment: Enrichment): Scalar | object => {
    const copy: Props = {};
    for (const name of dictionaryValueColumns(schema, enrichment)) copy[name] = row[name];
    return copy;
  };
}
