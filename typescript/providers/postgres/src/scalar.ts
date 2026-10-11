export type Scalar = string | number | boolean | bigint | null;
export type Wire = Scalar | Date | Uint8Array;
export type Cells = Record<string, Scalar>;
