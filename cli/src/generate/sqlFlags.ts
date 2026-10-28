import type { CaseFormat } from "./casing.ts";

export const letterCases = ["Upper", "Lower", "Auto"] as const;

export type LetterCase = (typeof letterCases)[number];

export type SqlFlags = {
  pluralizeTableNames?: boolean;
  tableNames?: CaseFormat;
  columnNames?: CaseFormat;
  objectNames?: LetterCase;
  sqlKeywords?: LetterCase;
  batchSeparator?: string;
};

export const parseLetterCaseFlag = (raw: string, name: string): LetterCase => {
  const parsed = letterCases.find((format) => format.toLowerCase() === raw.toLowerCase());
  if (parsed === undefined) throw new Error(`${name} must be one of [${letterCases.join(", ")}] (got "${raw}").`);
  return parsed;
};

export const overlaySqlFlags = (settings: Record<string, string>, flags: SqlFlags): Record<string, string> => ({
  ...settings,
  ...(flags.pluralizeTableNames === undefined
    ? {}
    : { "datasource.pluralize_datatable_names": flags.pluralizeTableNames ? "true" : "false" }),
  ...(flags.tableNames === undefined ? {} : { "datasource.casing.types": flags.tableNames }),
  ...(flags.columnNames === undefined ? {} : { "datasource.casing.fields": flags.columnNames }),
  ...(flags.objectNames === undefined ? {} : { "datasource.casing.objects": flags.objectNames }),
  ...(flags.sqlKeywords === undefined ? {} : { "datasource.casing.keywords": flags.sqlKeywords }),
  ...(flags.batchSeparator === undefined ? {} : { "datasource.batch_separator": flags.batchSeparator }),
});

export const sqlFlagsOf = (flags: SqlFlags): SqlFlags => ({
  pluralizeTableNames: flags.pluralizeTableNames,
  tableNames: flags.tableNames,
  columnNames: flags.columnNames,
  objectNames: flags.objectNames,
  sqlKeywords: flags.sqlKeywords,
  batchSeparator: flags.batchSeparator,
});
