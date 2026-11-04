import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import { EnrichmentPropertyType } from "./EnrichmentPropertyType.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import { identityKey } from "./identity.ts";
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
      const key = this.sourceKey(parent, enrichment, sources, index);
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
      const key = this.sourceKey(parent, enrichment, sources, index);
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
      const key = this.sourceKey(parent, enrichment, sources, index);
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
    const mapKey = mapKeyName(enrichment.relatedKey, probe(via));
    for (const [index, parent] of parents.entries()) {
      const key = this.sourceKey(parent, enrichment, sources, index);
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
          : this.copyValue(item, (parent as Props)[enrichment.destinationProperty], enrichment, mapKey);
      }
      Object.assign(parent, { [enrichment.destinationProperty]: map });
    }
  }

  protected sourceKey(
    parent: T,
    enrichment: Enrichment,
    sources: (Row | undefined)[] | undefined,
    index: number,
  ): Scalar | object | undefined {
    const cell = sources?.[index]?.[enrichment.sourceProperty];
    const raw = cell !== undefined && cell !== null ? cell : this.props(parent)[enrichment.sourceProperty];
    return this.lookupKey(raw, enrichment.relatedKey);
  }

  protected props = <TItem>(item: TItem): Props => item as never as Props;

  protected keyOf = (key: Scalar | object, name = ""): string => String(this.lookupKey(key, name));

  protected lookupKey = (value: Scalar | object | undefined, name: string): Scalar | object | undefined =>
    identityKey(value, name);

  private copyValue = (
    item: object,
    dest: Scalar | object | undefined,
    enrichment: Enrichment,
    mapKey: string,
  ): Scalar | object => {
    const row = this.props(item);
    if (this.stringMap(dest)) return (row.Value ?? row.value ?? null) as Scalar;
    const names = this.destValueNames(dest);
    const copy: Props = {};
    for (const [name, value] of Object.entries(row)) {
      if (names.size > 0 ? names.has(name) : name !== mapKey && name !== enrichment.relatedKey)
        copy[name] = value;
    }
    return copy;
  };

  private stringMap = (dest: Scalar | object | undefined): boolean => {
    if (!dest || typeof dest !== "object") return false;
    const values = Object.values(dest);
    return values.length > 0 && values.every((value) => typeof value === "string");
  };

  private destValueNames = (dest: Scalar | object | undefined): Set<string> => {
    const sample = dest && typeof dest === "object"
      ? Object.values(dest).find((value) => value !== null && typeof value === "object" && !Array.isArray(value))
      : undefined;
    if (!sample || typeof sample !== "object") return new Set();
    const ctor = sample.constructor;
    const blank = typeof ctor === "function" && ctor !== Object ? new (ctor as new () => object)() : sample;
    return new Set(Object.keys(this.props(blank)));
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

const probe = (via?: { type: new () => object }): object | undefined => {
  try {
    return via ? new via.type() : undefined;
  } catch {
    return undefined;
  }
};

const mapKeyName = (relatedKey: string, child?: object): string =>
  child && "Key" in child ? "Key"
    : child && "key" in child ? "key"
      : child && "Name" in child ? "Name"
        : child && "name" in child ? "name"
          : relatedKey.includes("_") || relatedKey[0] !== relatedKey[0]?.toUpperCase() ? "key" : "Key";

