export type Pair = { from: string; to: string };

export type PlannedEnrichment = {
  destination: string;
  source: string;
  relatedKey: string;
  relatedProperty?: string;
  kind: "list" | "object" | "linked";
  via: string;
  viaType: string;
  link?: string;
  linkType?: string;
  parentFk?: string;
  relatedFk?: string;
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
  enrichments: PlannedEnrichment[];
};

export const repositoryFile = (dialect: "csharp" | "typescript" | "rust", model: RepositoryModel): string =>
  dialect === "csharp"
    ? `${model.typeName}Repository.cs`
    : dialect === "typescript"
      ? `${model.specName.replaceAll("_", "-")}-repository.ts`
      : `${model.specName}_repository.rs`;

export const childField = (dialect: "csharp" | "typescript" | "rust", specName: string, typeName: string): string =>
  dialect === "csharp" ? typeName : dialect === "typescript" ? camel(specName) : specName;

export const childClass = (typeName: string): string => `${typeName}Repository`;

export type LinkedEnrichment = PlannedEnrichment & {
  link: string;
  linkType: string;
  parentFk: string;
  relatedFk: string;
};

export const isLinked = (item: PlannedEnrichment): item is LinkedEnrichment => item.kind === "linked";

export const ownedChildren = (model: RepositoryModel): Array<{ specName: string; typeName: string }> => {
  const rows = new Map<string, string>();
  for (const item of model.enrichments) {
    rows.set(item.via, item.viaType);
    if (isLinked(item)) rows.set(item.link, item.linkType);
  }
  return [...rows].map(([specName, typeName]) => ({ specName, typeName }));
};

const camel = (name: string): string =>
  name.split("_").map((word, index) => (index === 0 ? word : word.charAt(0).toUpperCase() + word.slice(1))).join("");
