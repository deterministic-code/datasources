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
