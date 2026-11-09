import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { EnrichmentPropertyType } from "./EnrichmentPropertyType.ts";
import type { Schema } from "./Schema.ts";

export type Association = {
  parent: string;
  related: string;
};

export type Enrichment = {
  destinationProperty: string;
  sourceProperty: string;
  relatedKey: string;
  relatedProperty?: string;
  type?: EnrichmentPropertyType;
  association?: Association;
  dictionaryKey?: string;
};

export const dictionaryMapKey = (enrichment: Enrichment): string => {
  if (enrichment.dictionaryKey === undefined)
    throw new DatasourceError(`${enrichment.destinationProperty} requires "dictionaryKey"`);
  return enrichment.dictionaryKey;
};

export const dictionaryValueColumns = (schema: Schema, enrichment: Enrichment): string[] => {
  const mapKey = dictionaryMapKey(enrichment);
  return Object.entries(schema.columns)
    .filter(([name, column]) => column.isId !== true && name !== enrichment.relatedKey && name !== mapKey)
    .map(([name]) => name);
};

type Props = Record<string, Scalar | object>;

export const mapPayload = (
  value: Scalar | object | undefined,
  enrichment?: Enrichment,
  schema?: Schema,
): object[] => {
  if (Array.isArray(value)) return value.filter((item) => item !== null && typeof item === "object");
  if (value === undefined || value === null || typeof value !== "object") return [];
  if (enrichment?.type !== "dictionary") return [];
  const columns = schema ?? new Schema({});
  const keyName = dictionaryMapKey(enrichment);
  return Object.entries(value)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([key, cell]) => {
      if (enrichment.relatedProperty)
        return { [keyName]: key, [enrichment.relatedProperty]: cell };
      const row: Props = { [keyName]: key };
      if (cell !== null && typeof cell === "object" && !Array.isArray(cell))
        for (const name of dictionaryValueColumns(columns, enrichment))
          row[name] = (cell as Props)[name];
      return row;
    });
};
