import { isAbsolute, join } from "node:path";
import { generate as generateSql } from "@deterministic-code/datasources-code-generators/generate-sql";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import type { Flags, MigrationsCreateCommand } from "../parse.ts";
import type { Deps } from "../run.ts";
import { loadSpec } from "./loadSpec.ts";
import { providerByName } from "./providers.ts";
import { withStoredProcedures } from "./storedProcedures.ts";
import { writeEntries } from "./writeEntries.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

const afterMigrations = (filename: string): string => filename.replace(/^[^/]+\/migrations\//, "");

export const applyMigrations = async (
  command: MigrationsCreateCommand,
  flags: Flags,
  deps: Deps,
): Promise<void> => {
  providerByName(command.provider);
  const loaded = await loadSpec(deps.fs, at(flags.cwd, command.input));
  const listed = [
    ...new Set(
      loaded.settings["backend.datasources"]
        .split(",")
        .map((name) => name.trim())
        .filter((name) => name !== ""),
    ),
  ];
  if (command.migrationsPath !== undefined && listed.length > 1)
    throw new Error("--migrations-path requires a single SQL dialect in backend.datasources");
  const entries = await generateSql({
    reader: memoryReader(loaded.files),
    settings: withStoredProcedures(loaded.settings, command.storedProcedures),
  });
  const remapped =
    command.migrationsPath === undefined
      ? entries
      : entries.map((entry) => ({ ...entry, filename: afterMigrations(entry.filename) }));
  const root = at(flags.cwd, command.migrationsPath ?? command.migrationsRoot);
  if (flags.dryRun) {
    for (const entry of remapped) deps.log(join(root, entry.filename));
    return;
  }
  for (const path of await writeEntries(deps.fs, root, remapped)) deps.log(path);
};
