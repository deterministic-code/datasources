import { enrichmentMatches, mapPayload, type Enrichment } from "./Enrichment.ts";
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

  objectSources = (entries: readonly StoredEnrichment<T>[]): string[] =>
    entries
      .filter((entry) => (entry.enrichment.type ?? "object") === "object" && !entry.enrichment.association)
      .map((entry) => entry.enrichment.sourceProperty);

  sources = (entries: readonly StoredEnrichment<T>[], column: (property: string) => string): Set<string> =>
    new Set(this.objectSources(entries).flatMap((source) => [source, column(source)]));

  destination = (entries: readonly StoredEnrichment<T>[], name: string): boolean =>
    entries.some((entry) => entry.enrichment.destinationProperty === name);

  lookup = async (
    entries: readonly StoredEnrichment<T>[],
    items: T[],
    record: Cells,
    schema: Schema,
    column: (property: string) => string,
  ): Promise<void> => {
    for (const source of this.objectSources(entries)) {
      const mapped = column(source);
      if (!this.presentCell(record[source]) && this.presentCell(record[mapped])) record[source] = record[mapped]!;
    }
    await this.resolveLookups(entries, items, record, schema);
    for (const source of this.objectSources(entries)) {
      if (!this.presentCell(record[source])) continue;
      const mapped = column(source);
      if (mapped !== source) delete record[mapped];
    }
  };

  async resolveLookups(entries: readonly StoredEnrichment<T>[], items: T[], record: Cells, schema: Schema): Promise<void> {
    for (const entry of entries) {
      const { enrichment } = entry;
      const via = entry.via ?? entry.persist;
      if (enrichment.relatedProperty && enrichment.association && via)
        await this.resolveLinked(via, enrichment, items);
    }
    for (const item of items) {
      for (const entry of entries) {
        const { enrichment } = entry;
        if ((enrichment.type ?? "object") !== "object" || enrichment.association) continue;
        if (enrichment.relatedProperty) continue;
        if (this.presentCell(record[enrichment.sourceProperty])) continue;
        const nested = this.getCell(item, enrichment.destinationProperty);
        if (nested === null || nested === undefined || typeof nested !== "object") continue;
        const nestedId = this.getCell(nested, enrichment.relatedKey);
        if (!this.presentCell(nestedId)) continue;
        record[enrichment.sourceProperty] = nestedId;
      }
    }
    for (const item of items) {
      for (const entry of entries) {
        const { enrichment } = entry;
        if (!enrichment.relatedProperty) continue;
        const via = entry.via ?? entry.persist;
        if (!via) continue;
        if (enrichment.association) continue;
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

  /**
   * Fills the missing related keys of every item's linked rows from their names, with one batched
   * lookup per enrichment instead of one per row.
   */
  private resolveLinked = async <TRelated>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    items: T[],
  ): Promise<void> => {
    const property = enrichment.relatedProperty;
    if (!property) return;
    const rows: { related: TRelated; name: Scalar }[] = [];
    for (const item of items)
      for (const related of this.payloadOf<TRelated, T>(item, enrichment.destinationProperty)) {
        if (this.presentCell(this.getCell(related, enrichment.relatedKey))) continue;
        const name = this.getCell(related, property);
        if (this.presentCell(name)) rows.push({ related, name });
      }
    if (rows.length === 0) return;
    const names = [...new Map(rows.map(({ name }) => [String(name), name])).values()];
    const found = await this.lookupBatch(via, property, names, enrichment.relatedKey);
    for (const { related, name } of rows) {
      const id = found.get(String(name));
      if (!this.presentCell(id))
        throw new DatasourceError(`Unknown ${enrichment.destinationProperty} "${String(name)}"`);
      Object.assign(related, { [enrichment.relatedKey]: id });
    }
  };

  /**
   * One `IN` read per chunk of names; the first row per name wins, as in a single-row lookup. A name
   * the batch read does not return falls back to the single-name read, so results match.
   */
  protected lookupBatch = async <TRelated>(
    via: IReadonlyRepository<TRelated>,
    property: string,
    names: Scalar[],
    key: string,
  ): Promise<Map<string, Scalar | object | undefined>> => {
    const found = new Map<string, Scalar | object | undefined>();
    for (let offset = 0; offset < names.length; offset += 500)
      for await (const item of via.read(new Where(new Operator.In(property, names.slice(offset, offset + 500))))) {
        const name = this.getCell(item, property);
        if (name === undefined || name === null || found.has(String(name))) continue;
        found.set(String(name), this.getCell(item, key));
      }
    for (const name of names) {
      if (found.has(String(name))) continue;
      const item = await via.readSingle(new Where(new Operator.Equals(property, name)));
      if (item) found.set(String(name), this.getCell(item, key));
    }
    return found;
  };

  collectChildren<TChild>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): TChild[] {
    const children = this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment, writer);
    if (children.length === 0) return children;
    for (const match of enrichmentMatches(enrichment)) {
      const key = this.requiredValue(parent, match.sourceProperty, enrichment.destinationProperty);
      for (const child of children) this.stamp(child, match.relatedKey, key);
    }
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
  ): TChild[] => mapPayload(this.getCell(parent, into), enrichment, writer?.schema) as TChild[];

  protected stamp = <TItem extends object>(
    related: TItem,
    by: string,
    key: Scalar | object | undefined,
  ): void => {
    if (key === undefined || key === null || typeof key === "object") return;
    const current = this.getCell(related, by);
    if (current !== null && typeof current === "object" && !Array.isArray(current)) {
      const target = current as Props;
      const name = by in target ? by : "Id" in target ? "Id" : "id";
      Object.assign(target, { [name]: key });
      return;
    }
    Object.assign(related, { [by]: key });
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

  protected requiredValue = <TParent>(parent: TParent, name: string, into: string): Scalar => {
    const key = this.getCell(parent, name);
    if (!this.presentCell(key)) throw new DatasourceError(`${into} requires "${name}"`);
    return key;
  };

  protected requiredKey = <TParent>(parent: TParent, enrichment: Enrichment): Scalar =>
    this.requiredValue(parent, enrichment.sourceProperty, enrichment.destinationProperty);
}
