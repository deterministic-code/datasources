import type { EnrichmentPropertyType } from "./EnrichmentPropertyType.ts";
import { Schema } from "./Schema.ts";

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
};

export const valueObjectNames = (value: object): string[] =>
  Object.values(value)
    .filter((cell): cell is object => cell !== null && typeof cell === "object" && !Array.isArray(cell))
    .flatMap((cell) => Object.keys(cell));

export const dictionaryMapKey = (
  enrichment: Enrichment,
  schema: Schema,
  extraSkip: readonly string[] = [],
): string => {
  const skip = new Set(
    ["id", enrichment.relatedKey, ...(enrichment.relatedProperty === undefined ? [] : [enrichment.relatedProperty]), ...extraSkip].map(
      (name) => name.toLowerCase(),
    ),
  );
  const leftover = Object.entries(schema.columns)
    .filter(([name, column]) => !skip.has(name.toLowerCase()) && column.hasDefault !== true)
    .map(([name]) => name);
  if (leftover.length === 0) throw new Error("dictionary map key is missing on the related object");
  return leftover[0]!;
};
