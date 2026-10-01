import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";

type Props = Record<string, Scalar | object>;

export abstract class CreateEnrichmentStrategy<T> {
  declare private readonly subject: T;

  abstract createOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>;

  abstract createLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>;

  protected getCell = (item: object, name: string): Scalar | object | undefined => (item as Props)[name];

  protected presentCell = (value: Scalar | object | undefined): value is Scalar =>
    value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

  protected payloadOf = (parent: object, into: string): object[] => {
    const value = this.getCell(parent, into);
    return value === undefined || value === null || typeof value === "string"
      ? []
      : Array.isArray(value)
        ? value.filter((item) => item !== null && typeof item === "object")
        : [];
  };

  protected stampCell = (related: object, by: string, key: Scalar | object | undefined): void => {
    if (key === undefined || key === null || typeof key === "object") return;
    const current = (related as Props)[by];
    if (current !== null && typeof current === "object" && !Array.isArray(current)) {
      const target = current as Props;
      const id = "Id" in target ? "Id" : "id";
      target[id] = key;
      return;
    }
    Object.assign(related, { [by]: key });
  };

  protected relatedIds = (parent: object, enrichment: Enrichment): Scalar[] => {
    const ids: Scalar[] = [];
    const seen = new Set<string>();
    for (const related of this.payloadOf(parent, enrichment.destinationProperty)) {
      const id = this.getCell(related, enrichment.relatedKey);
      if (!this.presentCell(id)) throw new DatasourceError(`${enrichment.destinationProperty} requires "${enrichment.relatedKey}"`);
      const key = String(id);
      if (seen.has(key)) continue;
      seen.add(key);
      ids.push(id);
    }
    return ids;
  };

  protected requiredKey = (parent: object, enrichment: Enrichment): Scalar => {
    const key = this.getCell(parent, enrichment.sourceProperty);
    if (!this.presentCell(key)) throw new DatasourceError(`${enrichment.destinationProperty} requires "${enrichment.sourceProperty}"`);
    return key;
  };
}
