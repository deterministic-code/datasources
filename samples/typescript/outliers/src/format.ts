import type { MappingCell } from "./mappingCell.ts";

export type TreeNode = { name: string; children: TreeNode[] };
export type RegionNode = { regionId: number; categoryId: number; name: string; children: RegionNode[] };
export type GraphNode = { value: string; edges: { value: string }[] };

export const treeLine = (node: TreeNode): string => {
  const kids = [...node.children].sort((left, right) => left.name.localeCompare(right.name));
  return [node.name, ...kids.map(treeLine)].join(" ");
};

export const regionLine = (node: RegionNode): string => {
  const kids = [...node.children].sort((left, right) => left.name.localeCompare(right.name));
  return [`${node.regionId}-${node.categoryId}/${node.name}`, ...kids.map(regionLine)].join(" ");
};

export const graphLine = (node: GraphNode): string => {
  const edges = [...node.edges].map((edge) => edge.value).sort();
  return edges.length === 0 ? node.value : `${node.value} ${edges.join(",")}`;
};

export const mapError = (method: string, message: string): string[] => [`map ${method} error ${message}`];

export const mapLines = (method: string, row: MappingCell): string[] =>
  cells(row).map(([column, value]) =>
    value.length === 0 ? `map ${method} ${column}` : `map ${method} ${column} ${value}`,
  );

export const transcript = (
  trees: TreeNode[],
  regions: RegionNode[],
  graphs: GraphNode[],
  maps: string[] = [],
): string[] =>
  [
    ...trees.map((row) => `tree ${treeLine(row)}`),
    ...regions.map((row) => `region ${regionLine(row)}`),
    ...graphs.map((row) => `graph ${graphLine(row)}`),
    ...maps,
  ].sort();

const cells = (row: MappingCell): [string, string][] => [
  ["id", textNumber(row.id)],
  ["text_to_string", row.textToString],
  ["text_to_integer", textNumber(row.textToInteger)],
  ["text_to_decimal", String(row.textToDecimal)],
  ["text_to_datetime", textDate(row.textToDatetime)],
  ["text_to_binary", textBytes(row.textToBinary)],
  ["text_to_guid", row.textToGuid],
  ["int_to_integer", textNumber(row.intToInteger)],
  ["int_to_float", textNumber(row.intToFloat)],
  ["int_to_decimal", String(row.intToDecimal)],
  ["int_to_boolean", row.intToBoolean ? "true" : "false"],
  ["float_to_integer", textNumber(row.floatToInteger)],
  ["float_to_float", textNumber(row.floatToFloat)],
  ["float_to_decimal", String(row.floatToDecimal)],
  ["decimal_to_integer", textNumber(row.decimalToInteger)],
  ["decimal_to_float", textDecimalPlaces(row.decimalToFloat)],
  ["decimal_to_decimal", String(row.decimalToDecimal)],
  ["bool_to_boolean", row.boolToBoolean ? "true" : "false"],
  ["datetime_to_datetime", textDate(row.datetimeToDatetime)],
  ["binary_to_binary", textBytes(row.binaryToBinary)],
  ["binary_to_guid", row.binaryToGuid],
  ["guid_to_string", row.guidToString],
  ["guid_to_guid", row.guidToGuid],
];

const textNumber = (value: number): string => String(value);

const textDecimalPlaces = (value: number | string): string => {
  const number = Number(value);
  return Number.isFinite(number) ? number.toFixed(8) : String(value);
};

const textDate = (value: Date): string => (value.getTime() === 0 ? "" : value.toISOString());

const textBytes = (value: Uint8Array | string): string => {
  if (typeof value === "string") return Buffer.from(value, "base64").toString("hex");
  return Buffer.from(value).toString("hex");
};
