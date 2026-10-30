import {
  KeywordCasing,
  StatementNaming,
  type Casing,
} from "@deterministic-code/datasources";
import {
  createCasingStrategy,
  type ICasingStrategy,
} from "@deterministic-code/generators-common/casing-strategy";
import { createDatasourceNaming } from "@deterministic-code/generators-common/datasource-naming";

const DATASOURCE_CASING = "datasource.casing";
const UPPER_LOWER = ["upper", "lower"] as const;
type UpperLower = (typeof UPPER_LOWER)[number];

export type PackCasing = ICasingStrategy & {
  naming: StatementNaming
  isVerbatim: (mapping: string) => boolean
  /** Always-plural routine English (`find_contacts`); ignores the table flag. */
  pluralTableName: (entity: string) => string
  /** Snake plural stem for routine names (`notification_types`). */
  pluralStem: (entity: string) => string
  routineName: (stem: string) => string
  fileBase: (stem: string) => string
  filePath: (stem: string) => string
  directory: (entity: string) => string
};

const parseUpperLower = (
  raw: string | undefined,
  path: string,
  fallback: UpperLower | "preserve",
): UpperLower | "preserve" => {
  if (raw === undefined || raw === "") return fallback;
  const parsed = raw.toLowerCase();
  if (parsed === "auto") return fallback;
  if (parsed === "upper" || parsed === "lower") return parsed;
  throw new Error(
    `${path} must be one of [${UPPER_LOWER.join(", ")}] (got "${raw}").`,
  );
};

const identifier = (settings: Record<string, string>, path: string): Casing => {
  const raw = settings[path];
  if (raw === undefined || raw === "" || raw.toLowerCase() === "auto") return "snakeCase";
  switch (raw.toLowerCase()) {
    case "camel":
      return "camelCase";
    case "pascal":
      return "pascalCase";
    case "snake":
      return "snakeCase";
    case "kebab":
      return "kebabCase";
    default:
      throw new Error(
        `${path} must be one of [Camel, Pascal, Snake, Kebab] (got "${raw}").`,
      );
  }
};

/** Datasource naming + SQL-only keyword/object letter-case. */
export const createCasing = (
  settings: Record<string, string>,
): PackCasing => {
  const naming = createDatasourceNaming(settings);
  const alwaysPlural = createDatasourceNaming({
    ...settings,
    "datasource.pluralize_datatable_names": "true",
  });
  const casing = createCasingStrategy("sql", settings, {
    prefix: DATASOURCE_CASING,
  });
  const objects = parseUpperLower(
    settings[`${DATASOURCE_CASING}.objects`],
    `${DATASOURCE_CASING}.objects`,
    "preserve",
  );
  const keywords = parseUpperLower(
    settings[`${DATASOURCE_CASING}.keywords`],
    `${DATASOURCE_CASING}.keywords`,
    "upper",
  ) as UpperLower;
  const types = identifier(settings, `${DATASOURCE_CASING}.types`);
  const strategy = new StatementNaming({
    tableNameCasing: types,
    fieldNameCasing: identifier(settings, `${DATASOURCE_CASING}.fields`),
    constraintNameCasing: types,
    triggerNameCasing: types,
    keywordCasing: keywords === "lower" ? KeywordCasing.Lower : KeywordCasing.Upper,
    pluralizeTables: settings["datasource.pluralize_datatable_names"] !== "false",
    objects: objects === "upper" || objects === "lower" ? objects : "preserve",
  });
  const objectIdent = (stem: string, converted: string): string =>
    naming.isVerbatim(stem) ? converted : strategy.letter(converted);
  const fileBase = (stem: string): string => casing.convertFileName(stem);
  return {
    naming: strategy,
    convertFileName: (text) => casing.convertFileName(text),
    convertTypes: (text) => casing.convertTypes(text),
    convertFields: (text) => casing.convertFields(text),
    convertDirectories: (text) => casing.convertDirectories(text),
    isVerbatim: naming.isVerbatim,
    pluralTableName: (entity) =>
      objectIdent(entity, alwaysPlural.resolveTable(entity)),
    pluralStem: (entity) => alwaysPlural.physicalStem(entity),
    routineName: (stem) => objectIdent(stem, naming.tableName(stem)),
    fileBase,
    filePath: (stem) => `${fileBase(stem)}.sql`,
    directory: (entity) => casing.convertDirectories(entity),
  };
};
