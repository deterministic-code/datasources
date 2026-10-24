import type { RepositoryModel, RepositoryOptionsModel } from "./RepositoryModel.ts";

const same = (left: string[], right: string[]): boolean =>
  left.length === right.length && left.every((name, index) => name === right[index]);

const samePairs = (left: RepositoryOptionsModel["propertyMappings"], right: RepositoryOptionsModel["propertyMappings"]): boolean =>
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

export const quote = (value: string): string => value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"");

export const pad = (text: string, spaces: number): string => {
  const prefix = " ".repeat(spaces);
  return text.split("\n").map((line) => `${prefix}${line}`).join("\n");
};

const csharpList = (names: string[]): string =>
  `[${names.map((name) => `"${quote(name)}"`).join(", ")}]`;

const tsList = (names: string[]): string =>
  `[${names.map((name) => `"${quote(name)}"`).join(", ")}]`;

const rustList = (names: string[]): string =>
  `vec![${names.map((name) => `"${quote(name)}".into()`).join(", ")}]`;

export const writeList = (dialect: "csharp" | "typescript" | "rust", names: string[]): string =>
  dialect === "csharp" ? csharpList(names) : dialect === "typescript" ? tsList(names) : rustList(names);
