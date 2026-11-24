import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { dictionaryMapKey, dictionaryValueColumns, enrichmentMatches, type Enrichment } from "./Enrichment.ts";
import { EnrichmentPropertyType } from "./EnrichmentPropertyType.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { Schema } from "./Schema.ts";
import { identityKey } from "./identity.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

export class DefaultReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  protected batchSize = 500;

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
      const key = this.sourceKey(parent, enrichment, sources, index);
      if (key === undefined || key === null) continue;
      const related: TRelated[] = [];
      for await (const row of link.read(
        new Where(new Operator.Equals(association.parent, this.scalar(key))),
      )) {
        const id = this.props(row)[association.related];
        if (id === undefined || id === null) continue;
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

  protected async applyList<TRelated extends object>(
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

  protected async applyDictionary<TRelated extends object>(
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
          : this.copyValue(row, schema, enrichment);
      }
      Object.assign(parent, { [enrichment.destinationProperty]: map });
    }
  }

  protected async loadByKeys<TItem extends object>(
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

  protected distinctKeys(parents: T[], sources: (Row | undefined)[] | undefined, enrichment: Enrichment): Scalar[] {
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

  protected group<TRelated>(items: TRelated[], enrichment: Enrichment): Map<string, TRelated[]> {
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

  protected chunks = <TItem>(items: TItem[]): TItem[][] => {
    const out: TItem[][] = [];
    for (let offset = 0; offset < items.length; offset += this.batchSize)
      out.push(items.slice(offset, offset + this.batchSize));
    return out;
  };

  sourceValue(
    parent: T,
    name: string,
    sources: (Row | undefined)[] | undefined,
    index: number,
  ): Scalar | object | undefined {
    const cell = sources?.[index]?.[name];
    return cell !== undefined && cell !== null ? cell : this.props(parent)[name];
  }

  protected sourceKey(
    parent: T,
    enrichment: Enrichment,
    sources: (Row | undefined)[] | undefined,
    index: number,
  ): Scalar | object | undefined {
    return this.sourceValue(parent, enrichment.sourceProperty, sources, index);
  }

  matchKey(
    parent: object,
    enrichment: Enrichment,
    sources?: (Row | undefined)[],
    index = 0,
  ): string | undefined {
    const parts: string[] = [];
    for (const match of enrichmentMatches(enrichment)) {
      const key = this.sourceValue(parent as T, match.sourceProperty, sources, index);
      if (key === undefined || key === null) return undefined;
      parts.push(this.keyOf(key));
    }
    return parts.join("\0");
  }

  rowMatchKey<TItem>(item: TItem, enrichment: Enrichment): string | undefined {
    const props = this.props(item);
    const parts: string[] = [];
    for (const match of enrichmentMatches(enrichment)) {
      const key = props[match.relatedKey];
      if (key === undefined || key === null) return undefined;
      parts.push(this.keyOf(key));
    }
    return parts.join("\0");
  }

  protected matchWhere(
    parent: T,
    enrichment: Enrichment,
    sources: (Row | undefined)[] | undefined,
    index: number,
  ): Where | undefined {
    const parts: Operator[] = [];
    for (const match of enrichmentMatches(enrichment)) {
      const key = this.sourceValue(parent, match.sourceProperty, sources, index);
      if (key === undefined || key === null) return undefined;
      parts.push(new Operator.Equals(match.relatedKey, this.scalar(key)));
    }
    const [first, ...rest] = parts;
    return first === undefined ? undefined : new Where(rest.reduce<Operator>((left, right) => new Operator.And(left, right), first));
  }

  protected props = <TItem>(item: TItem): Props => item as never as Props;

  keyOf = (key: Scalar | object, name = "Id"): string => String(this.lookupKey(key, name));

  protected lookupKey = (value: Scalar | object | undefined, name = "Id"): Scalar | object | undefined =>
    identityKey(value, name);

  private copyValue = (row: Props, schema: Schema, enrichment: Enrichment): Scalar | object => {
    const copy: Props = {};
    for (const name of dictionaryValueColumns(schema, enrichment)) copy[name] = row[name];
    return copy;
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

