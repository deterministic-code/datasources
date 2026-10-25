import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";

export type Pair = { from: string; to: string };

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
  updateProperties: string[];
  optionsByProvider?: Record<string, RepositoryOptionsModel>;
  optimisticConcurrency?: OptimisticConcurrency;
  enrichments: PlannedEnrichment[];
};

export type RepositoryOptionsModel = Pick<
  RepositoryModel,
  "table" | "ids" | "readProperties" | "createProperties" | "updateProperties" | "propertyMappings"
>;

export const repositoryFile = (ext: "cs" | "ts" | "rs", specName: string, casing: ICasingStrategy): string =>
  `${casing.convertFileName(`${specName}_repository`)}.${ext}`;

export const registryFile = (ext: "cs" | "ts" | "rs", casing: ICasingStrategy): string =>
  `${casing.convertFileName("repository_registry")}.${ext}`;

export const rustIdent = (stem: string): string => (/^[A-Za-z_][A-Za-z0-9_]*$/.test(stem) ? stem : stem.replaceAll("-", "_"));

export const rustModLine = (stem: string): string =>
  rustIdent(stem) === stem ? `pub mod ${stem};` : `#[path = "${stem}.rs"]\npub mod ${rustIdent(stem)};`;

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

const uncountable = new Set(["sheep", "fish", "series", "species", "deer", "news"]);
const irregular: Record<string, string> = {
  person: "people",
  child: "children",
  man: "men",
  woman: "women",
  mouse: "mice",
  goose: "geese",
  tooth: "teeth",
  foot: "feet",
  ox: "oxen",
};
const irregularSingular = Object.fromEntries(Object.entries(irregular).map(([singular, plural]) => [plural, singular]));
const vowel = (c: string) => "aeiou".includes(c);
const consonant = (c: string) => /[a-z]/.test(c) && !vowel(c);
const yies = (word: string) => word.length >= 4 && word.endsWith("ies") && consonant(word[word.length - 4]);
const consonantY = (word: string) => word.length >= 2 && word.endsWith("y") && consonant(word[word.length - 2]);
const sibilant = (word: string) =>
  word.endsWith("ch") ||
  word.endsWith("sh") ||
  word.endsWith("x") ||
  word.endsWith("z") ||
  word.endsWith("ss") ||
  (word.length >= 2 && word.endsWith("s") && !word.endsWith("es") && vowel(word[word.length - 2]));
const sibilantEs = (word: string) => word.length > 2 && word.endsWith("es") && sibilant(word.slice(0, -2));
const restore = (original: string, result: string) =>
  original[0] === original[0]?.toUpperCase() ? result[0]?.toUpperCase() + result.slice(1) : result;
const pluralWord = (word: string) => {
  const lower = word.toLowerCase();
  const plural =
    uncountable.has(lower) || lower in irregularSingular
      ? lower
      : irregular[lower] ??
        (yies(lower) || sibilantEs(lower)
          ? lower
          : sibilant(lower)
            ? `${lower}es`
            : consonantY(lower)
              ? `${lower.slice(0, -1)}ies`
              : lower.endsWith("s")
                ? lower
                : `${lower}s`);
  return restore(word, plural);
};

export const pluralPascal = (typeName: string): string => {
  const words = typeName.match(/[A-Z]+(?=[A-Z][a-z])|[A-Z]?[a-z]+|[A-Z]+|[0-9]+/g) ?? [typeName];
  const last = words[words.length - 1] as string;
  words[words.length - 1] = pluralWord(last);
  return words.join("");
};

export const pluralSnake = (specName: string): string => {
  const parts = specName.split("_");
  const last = parts[parts.length - 1] as string;
  parts[parts.length - 1] = pluralWord(last);
  return parts.join("_");
};

export const pluralCamel = (typeName: string): string => {
  const pascal = pluralPascal(typeName);
  return pascal.charAt(0).toLowerCase() + pascal.slice(1);
};
