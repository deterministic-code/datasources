import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { Cells } from "./converters/Cell.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const lookups = new WeakMap<object, Cells>();

export const mergeLookups = (item: object, record: Cells): void => {
  const extras = lookups.get(item);
  if (extras) Object.assign(record, extras);
};

export const lookupKeys = (item: object): Set<string> => new Set(Object.keys(lookups.get(item) ?? {}));

export abstract class CreateEnrichmentStrategy<T> {
  abstract createChildren<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void>;

  abstract createLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void>;

  async resolveLookups(entries: readonly StoredEnrichment<T>[], items: T[]): Promise<void> {
    for (const item of items) {
      for (const entry of entries) {
        const { enrichment } = entry;
        if ((enrichment.type ?? "object") !== "object" || enrichment.association || !enrichment.relatedProperty)
          continue;
        const name = this.getCell(item, enrichment.destinationProperty);
        if (!this.presentCell(name)) continue;
        if (this.presentCell(this.keyOf(this.getCell(item, enrichment.sourceProperty)))) continue;
        const via = entry.via ?? entry.persist;
        if (!via) continue;
        const found = await via.readSingle(new Where(new Operator.Equals(enrichment.relatedProperty, name)));
        if (!found)
          throw new DatasourceError(`Unknown ${enrichment.destinationProperty} "${String(name)}"`);
        const id = this.getCell(found, enrichment.relatedKey);
        if (!this.presentCell(id))
          throw new DatasourceError(`Unknown ${enrichment.destinationProperty} "${String(name)}"`);
        this.stamp(item as T & object, enrichment.sourceProperty, id);
      }
    }
  }

  collectChildren<TChild extends object>(_writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): TChild[] {
    const children = this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment);
    if (children.length === 0) return children;
    const key = this.requiredKey(parent, enrichment);
    for (const child of children) this.stampCell(child, enrichment.relatedKey, key);
    return children;
  }

  protected keyOf = (value: Scalar | object | undefined): Scalar | object | undefined => {
    if (value === undefined || value === null || typeof value !== "object") return value;
    const id = (value as Props).Id ?? (value as Props).id;
    return id === undefined || typeof id === "object" ? undefined : id;
  };

  protected getCell = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
    (item as never as Props)[name];

  protected presentCell = (value: Scalar | object | undefined): value is Scalar =>
    value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

  protected payloadOf = <TChild extends object, TParent>(
    parent: TParent,
    into: string,
    enrichment?: Enrichment,
  ): TChild[] => mapPayload(this.getCell(parent, into), enrichment) as TChild[];

  protected stamp = <TItem extends object>(related: TItem, by: string, key: Scalar | object | undefined): void => {
    if (key === undefined || key === null || typeof key === "object") return;
    const current = this.getCell(related, by);
    if (current !== null && typeof current === "object" && !Array.isArray(current)) {
      const target = current as Props;
      const id = "Id" in target ? "Id" : "id";
      target[id] = key;
      return;
    }
    if (by in related) {
      Object.assign(related, { [by]: key });
      return;
    }
    const extras = lookups.get(related) ?? {};
    extras[by] = key as Exclude<Scalar, object>;
    lookups.set(related, extras);
  };

  protected stampCell = <TItem extends object>(related: TItem, by: string, key: Scalar | object | undefined): void =>
    this.stamp(related, by, key);

  protected relatedIds = <TParent>(parent: TParent, enrichment: Enrichment): Scalar[] => {
    const ids: Scalar[] = [];
    const seen = new Set<string>();
    for (const related of this.payloadOf<object, TParent>(parent, enrichment.destinationProperty)) {
      const id = this.getCell(related, enrichment.relatedKey);
      if (!this.presentCell(id)) throw new DatasourceError(`${enrichment.destinationProperty} requires "${enrichment.relatedKey}"`);
      const key = String(id);
      if (seen.has(key)) continue;
      seen.add(key);
      ids.push(id);
    }
    return ids;
  };

  protected requiredKey = <TParent>(parent: TParent, enrichment: Enrichment): Scalar => {
    const key = this.getCell(parent, enrichment.sourceProperty);
    if (!this.presentCell(key)) throw new DatasourceError(`${enrichment.destinationProperty} requires "${enrichment.sourceProperty}"`);
    return key;
  };
}

const mapPayload = (value: Scalar | object | undefined, enrichment?: Enrichment): object[] => {
  if (value === undefined || value === null || typeof value === "string") return [];
  if (Array.isArray(value)) return value.filter((item) => item !== null && typeof item === "object");
  if (enrichment?.type !== "dictionary" || typeof value !== "object") return [];
  const keyName = mapKeyName(enrichment.relatedKey);
  const valueName = enrichment.relatedProperty ?? (keyName === "Key" ? "Value" : "value");
  return Object.entries(value as Record<string, Scalar | object>)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([key, cell]) =>
      cell !== null && typeof cell === "object" && !Array.isArray(cell)
        ? { [keyName]: key, ...(cell as object) }
        : { [keyName]: key, [valueName]: cell },
    );
};

const mapKeyName = (relatedKey: string): string =>
  relatedKey.includes("_") || relatedKey[0] !== relatedKey[0]?.toUpperCase() ? "key" : "Key";
