import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import type { Dialect } from "./GenerateRequest.ts";

export const caseFormats = ["Pascal", "Camel", "Snake", "Kebab", "Auto"] as const;

export type CaseFormat = (typeof caseFormats)[number];

export type CasingFlags = {
  fileNames?: CaseFormat;
  types?: CaseFormat;
  fields?: CaseFormat;
  directories?: CaseFormat;
};

const LITERAL_SEGMENTS = new Set(["csharp", "typescript", "rust", "sql", "custom", "__tests__", "frontend"]);

export const parseCaseFlag = (raw: string, name: string): CaseFormat => {
  const parsed = caseFormats.find((format) => format.toLowerCase() === raw.toLowerCase());
  if (parsed === undefined) throw new Error(`${name} must be one of [${caseFormats.join(", ")}] (got "${raw}").`);
  return parsed;
};

export const overlayCasingFlags = (
  settings: Record<string, string>,
  dialect: Dialect,
  flags: CasingFlags,
): Record<string, string> => {
  const prefix = `languages.${dialect}.casing`;
  return {
    ...settings,
    ...(flags.fileNames === undefined ? {} : { [`${prefix}.file_names`]: flags.fileNames }),
    ...(flags.types === undefined ? {} : { [`${prefix}.types`]: flags.types }),
    ...(flags.fields === undefined ? {} : { [`${prefix}.fields`]: flags.fields }),
    ...(flags.directories === undefined ? {} : { [`${prefix}.directories`]: flags.directories }),
  };
};

export const caseDirectories = (path: string, casing: ICasingStrategy): string =>
  path
    .split("/")
    .map((segment, index) =>
      index === 0 || LITERAL_SEGMENTS.has(segment) || segment.includes(".")
        ? segment
        : casing.convertDirectories(segment),
    )
    .join("/");
