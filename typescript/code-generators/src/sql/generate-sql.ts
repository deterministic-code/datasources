import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { generateSqlFor } from "./common/generate-sql.ts";
import { ensureDialect } from "./common/sql-dialect.ts";

/** DDL + custom migrations (+ stored procedures when enabled) for each dialect. */
export const generate = async (
  ctx: GenerateContext,
  dialects: readonly string[],
): Promise<GenerateEntry[]> => {
  const entries: GenerateEntry[] = [];
  for (const dialect of dialects) {
    entries.push(...(await generateSqlFor(await ensureDialect(dialect), ctx)));
  }
  return entries;
};
