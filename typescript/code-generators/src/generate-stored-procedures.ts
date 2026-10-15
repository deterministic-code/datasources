import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { generate as generateProceduresForDialects } from "./sql/generate-stored-procedures.ts";
import { dialectsFromSettings } from "./sql/common/sql-dialect.ts";

/** Stored-procedure migrations for dialects in `backend.datasources` that support them. */
export const generate = async (ctx: GenerateContext): Promise<GenerateEntry[]> =>
  generateProceduresForDialects(ctx, dialectsFromSettings(ctx.settings));
