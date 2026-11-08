export type Value =
  | string
  | number
  | boolean
  | bigint
  | Date
  | Uint8Array
  | null;

export type Cells = Record<string, Value>;
