import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { Cells } from "./converters/Value.ts";
import { identityKey } from "./identity.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Schema } from "./Schema.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

export abstract class CreateEnrichmentStrategy<T> {
  abstract createChildren<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void>;

  abstract createLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void>;

  async resolveLookups(entries: readonly StoredEnrichment<T>[], items: T[], record: Cells, schema: Schema): Promise<void> {
    for (const item of items) {
      for (const entry of entries) {
        const { enrichment } = entry;
        if (!enrichment.relatedProperty) continue;
        const via = entry.via ?? entry.persist;
        if (!via) continue;
        if (enrichment.association) {
          await this.resolveLinked(via, enrichment, item);
          continue;
        }
        if ((enrichment.type ?? "object") !== "object") continue;
        const dest = enrichment.destinationProperty;
        if (this.presentCell(record[enrichment.sourceProperty])) continue;
        const name = record[dest];
        if (!this.presentCell(name)) {
          const column = schema.columns[dest];
          if (column && !column.nullable) throw new DatasourceError(`Unknown ${dest}`);
          continue;
        }
        const found = await via.readSingle(new Where(new Operator.Equals(enrichment.relatedProperty, name)));
        if (!found)
          throw new DatasourceError(`Unknown ${dest} "${String(name)}"`);
        const id = this.getCell(found, enrichment.relatedKey);
        if (!this.presentCell(id))
          throw new DatasourceError(`Unknown ${dest} "${String(name)}"`);
        record[enrichment.sourceProperty] = id;
      }
    }
  }

  private resolveLinked = async <TRelated>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    item: T,
  ): Promise<void> => {
    const property = enrichment.relatedProperty;
    if (!property) return;
    for (const related of this.payloadOf<TRelated, T>(item, enrichment.destinationProperty)) {
      if (this.presentCell(this.getCell(related, enrichment.relatedKey))) continue;
      const name = this.getCell(related, property);
      if (!this.presentCell(name)) continue;
      const found = await via.readSingle(new Where(new Operator.Equals(property, name)));
      if (!found)
        throw new DatasourceError(`Unknown ${enrichment.destinationProperty} "${String(name)}"`);
      const id = this.getCell(found, enrichment.relatedKey);
      if (!this.presentCell(id))
        throw new DatasourceError(`Unknown ${enrichment.destinationProperty} "${String(name)}"`);
      Object.assign(related, { [enrichment.relatedKey]: id });
    }
  };

  collectChildren<TChild>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): TChild[] {
    const children = this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment, writer);
    if (children.length === 0) return children;
    const key = this.requiredKey(parent, enrichment);
    for (const child of children) this.stamp(child, enrichment.relatedKey, key);
    return children;
  }

  protected keyOf = (value: Scalar | object | undefined, name: string): Scalar | object | undefined => {
    const key = identityKey(value, name);
    return this.presentCell(key) ? key : undefined;
  };

  protected getCell = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
    (item as never as Props)[name];

  protected presentCell = (value: Scalar | object | undefined): value is Scalar =>
    value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

  protected payloadOf = <TChild, TParent>(
    parent: TParent,
    into: string,
    enrichment?: Enrichment,
    writer?: ICRUDRepository<TChild>,
  ): TChild[] => mapPayload(this.getCell(parent, into), enrichment, writer) as TChild[];

  protected stamp = <TItem extends object>(
    related: TItem,
    by: string,
    key: Scalar | object | undefined,
  ): void => {
    if (key === undefined || key === null || typeof key === "object") return;
    const current = this.getCell(related, by);
    if (current !== null && typeof current === "object" && !Array.isArray(current)) {
      const target = current as Props;
      const name = identityKey(target, by) !== target ? by : "Id" in target ? "Id" : "id";
      Object.assign(target, { [name]: key });
      return;
    }
    if (holds(related, by)) Object.assign(related, { [by]: key });
  };

  protected stampCell = <TItem extends object>(related: TItem, by: string, key: Scalar | object | undefined): void =>
    this.stamp(related, by, key);

  protected relatedIds = <TParent>(parent: TParent, enrichment: Enrichment): Scalar[] => {
    const ids: Scalar[] = [];
    const seen = new Set<string>();
    for (const related of this.payloadOf<Cells, TParent>(parent, enrichment.destinationProperty)) {
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

const blank = (related: object): object | undefined => {
  const ctor = related.constructor;
  if (typeof ctor !== "function" || ctor === Object) return undefined;
  try {
    return new (ctor as new () => object)();
  } catch {
    return undefined;
  }
};

const holds = (related: object, by: string): boolean => {
  if (Object.getPrototypeOf(related) === Object.prototype || by in related) return true;
  const instance = blank(related);
  return instance !== undefined && by in instance;
};

const mapPayload = (
  value: Scalar | object | undefined,
  enrichment?: Enrichment,
  writer?: { type: new () => object },
): object[] => {
  if (value === undefined || value === null || typeof value === "string") return [];
  if (Array.isArray(value)) return value.filter((item) => item !== null && typeof item === "object");
  if (enrichment?.type !== "dictionary" || typeof value !== "object") return [];
  const keyName = mapKeyName(enrichment.relatedKey, probe(writer));
  const valueName = enrichment.relatedProperty ?? (keyName === "Key" ? "Value" : "value");
  return Object.entries(value as Record<string, Scalar | object>)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([key, cell]) =>
      cell !== null && typeof cell === "object" && !Array.isArray(cell)
        ? { [keyName]: key, ...(cell as object) }
        : { [keyName]: key, [valueName]: cell },
    );
};

const probe = (writer?: { type: new () => object }): object | undefined => {
  try {
    return writer ? new writer.type() : undefined;
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
