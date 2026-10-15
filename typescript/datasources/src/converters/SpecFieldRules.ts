export type DecimalRule =
  | { kind: "optional"; type: string; fallback: string }
  | { kind: "required"; type: string };

export type SpecFieldRules = {
  dialectName?: string;
  stringUnlimited: string;
  stringSized: (n: number) => string;
  character?: "CHAR" | "NCHAR";
  decimal?: DecimalRule;
  binaryUnlimited?: string;
  binarySized?: (n: number) => string;
  occBinary?: string;
  rowVersion?: string;
};
