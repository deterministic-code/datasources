import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { generate as generateSqlForDialects } from "./sql/generate-sql.ts";
import { dialectsFromSettings } from "./sql/common/sql-dialect.ts";

/** DDL + custom migrations (+ stored procedures when enabled) for each dialect in `backend.datasources`. */
export const generate = async (ctx: GenerateContext): Promise<GenerateEntry[]> =>
  generateSqlForDialects(ctx, dialectsFromSettings(ctx.settings));
