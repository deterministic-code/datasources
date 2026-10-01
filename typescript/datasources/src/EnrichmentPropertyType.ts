export const EnrichmentPropertyType = {
  Object: "object",
  List: "list",
  Dictionary: "dictionary",
  Custom: "custom",
} as const;

export type EnrichmentPropertyType =
  (typeof EnrichmentPropertyType)[keyof typeof EnrichmentPropertyType];
