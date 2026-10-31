import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";

export type Pair = { from: string; to: string };

export type CreateZero = { name: string; kind: "string" | "boolean" };

export type PlannedEnrichment = {
  destination: string;
  source: string;
  relatedKey: string;
  relatedProperty?: string;
  kind: "list" | "object" | "linked" | "dictionary";
  via: string;
  viaType: string;
  link?: string;
  linkType?: string;
  parentFk?: string;
  relatedFk?: string;
};

export type OptimisticConcurrency = {
  property: string;
  increment: boolean;
};

export type RepositoryModel = {
  specName: string;
  typeName: string;
  typeHome: "view" | "datasource";
  entity: string;
  table: string;
  propertyMappings: Pair[];
  ids: string[];
  readProperties: string[];
  createProperties: string[];
  createZeros: CreateZero[];
  updateProperties: string[];
  optionsByProvider?: Record<string, RepositoryOptionsModel>;
  optimisticConcurrency?: OptimisticConcurrency;
  enrichments: PlannedEnrichment[];
  removeFields?: string[];
};

export type RepositoryOptionsModel = Pick<
  RepositoryModel,
  "table" | "ids" | "readProperties" | "createProperties" | "updateProperties" | "propertyMappings"
>;

const same = (left: string[], right: string[]): boolean =>
  left.length === right.length && left.every((name, index) => name === right[index]);

const samePairs = (left: Pair[], right: Pair[]): boolean =>
  left.length === right.length && left.every((pair, index) => pair.from === right[index]?.from && pair.to === right[index]?.to);

const sameOptions = (left: RepositoryOptionsModel, right: RepositoryOptionsModel): boolean =>
  left.table === right.table &&
  same(left.ids, right.ids) &&
  same(left.readProperties, right.readProperties) &&
  same(left.createProperties, right.createProperties) &&
  same(left.updateProperties, right.updateProperties) &&
  samePairs(left.propertyMappings, right.propertyMappings);

export const dialectsOf = (model: RepositoryModel): Array<[string, RepositoryOptionsModel]> =>
  Object.entries(model.optionsByProvider ?? {}).filter(([, planned]) => !sameOptions(model, planned));

export const childField = (casing: ICasingStrategy, specName: string): string => casing.convertFields(specName);

export const childClass = (typeName: string): string => `${typeName}Repository`;

export type LinkedEnrichment = PlannedEnrichment & {
  link: string;
  linkType: string;
  parentFk: string;
  relatedFk: string;
};

export const isLinked = (item: PlannedEnrichment): item is LinkedEnrichment => item.kind === "linked";

export const children = (model: RepositoryModel): Array<{ specName: string; typeName: string }> => {
  const rows = new Map<string, string>();
  for (const item of model.enrichments) {
    rows.set(item.via, item.viaType);
    if (isLinked(item)) rows.set(item.link, item.linkType);
  }
  return [...rows].map(([specName, typeName]) => ({ specName, typeName }));
};
