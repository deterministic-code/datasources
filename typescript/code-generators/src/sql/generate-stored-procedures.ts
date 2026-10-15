import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { generateProceduresForDialect } from "./common/generate-procedures.ts";
import { requireDialect } from "./common/sql-dialect.ts";

/** Stored-procedure migrations for dialects that support them. */
export const generate = async (
  ctx: GenerateContext,
  dialects: readonly string[],
): Promise<GenerateEntry[]> => {
  const entries: GenerateEntry[] = [];
  for (const dialect of dialects) {
    entries.push(...(await generateProceduresForDialect(requireDialect(dialect), ctx)));
  }
  return entries;
};
