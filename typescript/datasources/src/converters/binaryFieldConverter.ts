import { DatasourceError } from "../DatasourceError.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import { formatCell } from "./formatCell.ts";

export const binaryFieldConverter: FieldConverter = {
  to: (value) => {
    if (value === null) return null;
    if (typeof value === "string") return value;
    if (value instanceof Uint8Array) return encodeBase64(value);
    throw new DatasourceError(`Cannot convert ${formatCell(value)} to binary`);
  },
  from: (value) => {
    if (value === null) return null;
    if (value instanceof Uint8Array) return value;
    if (typeof value === "string") return decodeBase64(value);
    throw new DatasourceError(`Cannot convert ${formatCell(value)} to bytes`);
  },
};

const encodeBase64 = (bytes: Uint8Array): string =>
  Buffer.from(bytes).toString("base64");

const decodeBase64 = (value: string): Uint8Array =>
  new Uint8Array(Buffer.from(value, "base64"));
