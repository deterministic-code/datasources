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
