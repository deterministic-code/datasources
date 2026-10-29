import type { IMapper } from "./IMapper.ts";

const forward: Record<string, string> = {
  boolean: "boolean",
  Boolean: "boolean",
  number: "integer",
  Number: "integer",
  Date: "datetime",
  Uint8Array: "binary",
};

const reverse: Record<string, string> = {
  boolean: "boolean",
  integer: "number",
  datetime: "Date",
  binary: "Uint8Array",
};

export class DeterministicTypeMapper implements IMapper<string, string> {
  map = (value: string): string => forward[value] ?? "";
  reverseMap = (value: string): string => reverse[value] ?? "";
}
