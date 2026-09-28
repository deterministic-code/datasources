import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { defaultId } from "./createRepositoryOptions.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

export abstract class UpdateEnrichmentStrategy<T> {
  declare private readonly subject: T;

  abstract updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>;

  abstract updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>;

  async createChildren(
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> {
    for (const related of this.payloadOf(parent, enrichment.into)) {
      this.stampCell(related, enrichment.by, this.getCell(parent, enrichment.from));
      await writer.create(related);
    }
  }

  async createLinks(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: object): Promise<void> {
    const association = enrichment.association;
    if (!association) throw new DatasourceError("Missing association");
    const parentKey = this.requiredKey(parent, enrichment);
    for (const id of this.relatedIds(parent, enrichment))
      await writer.create({ [association.parent]: parentKey, [association.related]: id });
  }

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
    if (key !== undefined && key !== null && typeof key !== "object") Object.assign(related, { [by]: key });
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
    for (const related of this.payloadOf(parent, enrichment.into)) {
      const id = this.getCell(related, enrichment.by);
      if (!this.presentCell(id)) throw new DatasourceError(`${enrichment.into} requires "${enrichment.by}"`);
      const key = String(id);
      if (seen.has(key)) continue;
      seen.add(key);
      ids.push(id);
    }
    return ids;
  };

  protected requiredKey = (parent: object, enrichment: Enrichment): Scalar => {
    const key = this.getCell(parent, enrichment.from);
    if (!this.presentCell(key)) throw new DatasourceError(`${enrichment.into} requires "${enrichment.from}"`);
    return key;
  };
}
