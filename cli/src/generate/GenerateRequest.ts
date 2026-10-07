import type { IDeterministicReader } from "@deterministic-code/generators-common/deterministic-reader";

export const dialects = ["csharp", "typescript", "rust"] as const;

export type Dialect = (typeof dialects)[number];

export type GenerateRequest = {
  dialect: Dialect;
  reader: IDeterministicReader;
  settings: Record<string, string>;
};

export const isDialect = (value: string): value is Dialect =>
  (dialects as readonly string[]).includes(value);
