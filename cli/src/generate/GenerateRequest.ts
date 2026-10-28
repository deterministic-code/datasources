import type { IDeterministicReader } from "@deterministic-code/generators-common/deterministic-reader";
import type { EnvMap } from "../env.ts";

export const dialects = ["csharp", "typescript", "rust"] as const;

export type Dialect = (typeof dialects)[number];

export type GenerateRequest = {
  dialect: Dialect;
  reader: IDeterministicReader;
  settings: Record<string, string>;
  env?: EnvMap;
};

export const isDialect = (value: string): value is Dialect =>
  (dialects as readonly string[]).includes(value);
