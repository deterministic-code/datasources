import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { defaultId } from "./createCRUDRepositoryOptions.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

export abstract class UpdateEnrichmentStrategy<T> {
  declare private readonly subject: T;

  abstract updateOwned(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>;

  abstract updateLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void>;

  protected getCell = (item: object, name: string): Scalar | object | undefined => (item as Props)[name];

  protected presentCell = (value: Scalar | object | undefined): value is Scalar =>
    value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

  protected payloadOf = (parent: object, into: string, enrichment?: Enrichment): object[] =>
    mapPayload(this.getCell(parent, into), enrichment);

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

  protected hasKey = (item: object, keys: readonly string[]): boolean =>
    keys.length > 0 && keys.every((name) => this.presentCell(this.getCell(item, name)));

  protected keyOf = (item: object, keys: readonly string[]): string =>
    keys.map((name) => String(this.getCell(item, name) ?? "")).join("\u001f");

  protected namesOf = (writer: ICRUDRepository<object>, item: object): readonly string[] =>
    writer.ids.length > 0 ? writer.ids : defaultId(Object.keys(item));

  protected linkWhere = (
    association: NonNullable<Enrichment["association"]>,
    parentKey: Scalar,
    relatedKey: Scalar,
  ): Where =>
    new Where(
      new Operator.And(
        new Operator.Equals(association.parent, parentKey),
        new Operator.Equals(association.related, relatedKey),
      ),
    );

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

const mapPayload = (value: Scalar | object | undefined, enrichment?: Enrichment): object[] => {
  if (value === undefined || value === null || typeof value === "string") return [];
  if (Array.isArray(value)) return value.filter((item) => item !== null && typeof item === "object");
  if (enrichment?.type !== "dictionary" || typeof value !== "object") return [];
  const keyName = mapKeyName(enrichment.relatedKey);
  const valueName = enrichment.relatedProperty ?? (keyName === "Key" ? "Value" : "value");
  return Object.entries(value as Record<string, Scalar | object>).map(([key, cell]) =>
    cell !== null && typeof cell === "object" && !Array.isArray(cell)
      ? { [keyName]: key, ...(cell as object) }
      : { [keyName]: key, [valueName]: cell },
  );
};

const mapKeyName = (relatedKey: string): string =>
  relatedKey.includes("_") || relatedKey[0] !== relatedKey[0]?.toUpperCase() ? "key" : "Key";

