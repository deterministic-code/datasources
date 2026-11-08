import type { EnrichmentPropertyType } from "./EnrichmentPropertyType.ts";

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

export const relatedObjectNames = (
  repo?: { schema?: { columns: Record<string, unknown> }; type?: new () => object },
): string[] => {
  const fromType = (() => {
    try {
      return repo?.type === undefined ? [] : Object.keys(new repo.type() as object);
    } catch {
      return [];
    }
  })();
  if (fromType.length > 0) return fromType;
  return repo?.schema === undefined ? [] : Object.keys(repo.schema.columns);
};

export const valueObjectNames = (value: object): string[] =>
  Object.values(value)
    .filter((cell): cell is object => cell !== null && typeof cell === "object" && !Array.isArray(cell))
    .flatMap((cell) => Object.keys(cell));

export const dictionaryMapKey = (
  enrichment: Enrichment,
  names: readonly string[],
  extraSkip: readonly string[] = [],
): string => {
  const skip = new Set(
    ["id", enrichment.relatedKey, ...(enrichment.relatedProperty === undefined ? [] : [enrichment.relatedProperty]), ...extraSkip].map(
      (name) => name.toLowerCase(),
    ),
  );
  const leftover = names.filter((name) => !skip.has(name.toLowerCase()));
  if (leftover.length === 0) throw new Error("dictionary map key is missing on the related object");
  return leftover[0]!;
};
