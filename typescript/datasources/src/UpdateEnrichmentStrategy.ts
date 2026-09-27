import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { defaultId } from "./createRepositoryOptions.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

export const getCell = (item: object, name: string): Scalar | object | undefined => props(item)[name];

const setCell = (item: object, name: string, value: Scalar): void => {
  Object.assign(item, { [name]: value });
};

export const presentCell = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

export const payloadOf = (parent: object, into: string): object[] => {
  const value = getCell(parent, into);
  return value === undefined || value === null || typeof value === "string"
    ? []
    : Array.isArray(value)
      ? value.filter((item) => item !== null && typeof item === "object")
      : [];
};

export const stampCell = (related: object, by: string, key: Scalar | object | undefined): void => {
  if (key !== undefined && key !== null && typeof key !== "object") setCell(related, by, key);
};

export const hasKey = (item: object, keys: readonly string[]): boolean =>
  keys.length > 0 && keys.every((name) => presentCell(getCell(item, name)));

export const keyOf = (item: object, keys: readonly string[]): string =>
  keys.map((name) => String(getCell(item, name) ?? "")).join("\u001f");

export const namesOf = (writer: ICRUDRepository<object>, item: object): readonly string[] =>
  writer.ids.length > 0 ? writer.ids : defaultId(Object.keys(item));

export const linkWhere = (
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

export const relatedIds = (parent: object, enrichment: Enrichment): Scalar[] => {
  const ids: Scalar[] = [];
  const seen = new Set<string>();
  for (const related of payloadOf(parent, enrichment.into)) {
    const id = getCell(related, enrichment.by);
    if (!presentCell(id)) throw new DatasourceError(`${enrichment.into} requires "${enrichment.by}"`);
    const key = String(id);
    if (seen.has(key)) continue;
    seen.add(key);
    ids.push(id);
  }
  return ids;
};

export const requiredKey = (parent: object, enrichment: Enrichment): Scalar => {
  const key = getCell(parent, enrichment.from);
  if (!presentCell(key)) throw new DatasourceError(`${enrichment.into} requires "${enrichment.from}"`);
  return key;
};

export const createChildren = async (
  writer: ICRUDRepository<object>,
  enrichment: Enrichment,
  parent: object,
): Promise<void> => {
  for (const related of payloadOf(parent, enrichment.into)) {
    stampCell(related, enrichment.by, getCell(parent, enrichment.from));
    await writer.create(related);
  }
};

export const createLinks = async (
  writer: ICRUDRepository<object>,
  enrichment: Enrichment,
  parent: object,
): Promise<void> => {
  const association = enrichment.association;
  if (!association) throw new DatasourceError("Missing association");
  const parentKey = requiredKey(parent, enrichment);
  for (const id of relatedIds(parent, enrichment))
    await writer.create({ [association.parent]: parentKey, [association.related]: id });
};

export abstract class UpdateEnrichmentStrategy<T> {
  declare private readonly subject: T;

  abstract updateOwned(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>;

  abstract updateLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>;
}
