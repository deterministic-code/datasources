import { TypeConverter, type Cell } from "@deterministic-code/datasources";

const uuid =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

const hex = (bytes: Uint8Array): string =>
  [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");

const fromBytes = (bytes: Uint8Array): string => {
  const body = hex(bytes);
  return `${body.slice(0, 8)}-${body.slice(8, 12)}-${body.slice(12, 16)}-${body.slice(16, 20)}-${body.slice(20)}`;
};

const canonicalUuid = (from: Cell): Cell => {
  if (typeof from === "string") return uuid.test(from) ? from.toLowerCase() : from;
  if (from instanceof Uint8Array && from.length === 16) return fromBytes(from);
  return from;
};

export class UuidConverter extends TypeConverter<Cell, Cell> {
  convert = (from: Cell): Cell => canonicalUuid(from);
  reverseConvert = (to: Cell): Cell => canonicalUuid(to);
}
