import { isAbsolute, join } from "node:path";
import { generateSql } from "@deterministic-code/datasources-code-generators";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import type { Flags, MigrationsCreateCommand } from "../parse.ts";
import type { Deps } from "../io.ts";
import { destEnv } from "./destEnv.ts";
import { loadSpec } from "./loadSpec.ts";
import { providerByName } from "./providers.ts";
import { requireKnownReferenceTypes } from "./requireKnownReferenceTypes.ts";
import { overlaySqlFlags } from "./sqlFlags.ts";
import { sqlDialectsFromEnv } from "./sqlDialects.ts";
import { withStoredProcedures } from "./storedProcedures.ts";
import { writeEntries } from "./writeEntries.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

export const applyMigrations = async (
  command: MigrationsCreateCommand,
  flags: Flags,
  deps: Deps,
): Promise<void> => {
  providerByName(command.provider);
  const loaded = await loadSpec(deps.fs, at(flags.cwd, command.input));
  const reader = memoryReader(loaded.files);
  const settings = overlaySqlFlags(withStoredProcedures(loaded.settings, command.storedProcedures), command);
  requireKnownReferenceTypes(await DeterministicParser(reader).parse(settings));
  const entries = await generateSql(
    {
      reader,
      settings,
    },
    sqlDialectsFromEnv(await destEnv(deps.fs, flags.cwd)),
  );
  const root = at(flags.cwd, command.migrationsRoot);
  if (flags.dryRun) {
    for (const entry of entries) deps.log(join(root, entry.filename));
    return;
  }
  for (const path of await writeEntries(deps.fs, root, entries)) deps.log(path);
};
