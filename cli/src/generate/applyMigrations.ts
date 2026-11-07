import { basename, dirname, isAbsolute, join } from "node:path";
import { generateSql } from "@deterministic-code/datasources-code-generators";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import type { Flags, MigrationsCreateCommand } from "../parse.ts";
import type { Deps, Fs } from "../io.ts";
import { destEnv } from "./destEnv.ts";
import { loadSpec } from "./loadSpec.ts";
import { providerByName } from "./providers.ts";
import { requireKnownReferenceTypes } from "./requireKnownReferenceTypes.ts";
import { overlaySqlFlags } from "./sqlFlags.ts";
import { sqlDialectsFromEnv } from "./sqlDialects.ts";
import { withStoredProcedures } from "./storedProcedures.ts";
import { writeEntries } from "./writeEntries.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

const STALE_CUSTOM = /^\d{10}_custom_\d{3}_/;
const GENERATED_MIGRATION = /^(?:\d{4}_(?:initial_)?(?:up|down))\.sql$/;
const BASELINE_UP = ["0001_up.sql", "0001_initial_up.sql"] as const;
const downOf = (up: string): string => (up === "0001_up.sql" ? "0001_down.sql" : "0001_initial_down.sql");
const isBaselineUp = (name: string): name is (typeof BASELINE_UP)[number] =>
  BASELINE_UP.some((up) => up === name);

const preserveBaselineNames = async (
  fs: Fs,
  root: string,
  entries: GenerateEntry[],
): Promise<GenerateEntry[]> => {
  const dirs = new Set(
    entries.map((entry) => dirname(join(root, entry.filename))).filter((dir) => basename(dir) === "migrations"),
  );
  const renameByDir = new Map<string, Map<string, string>>();
  for (const dir of dirs) {
    const names = await fs.readdir(dir).catch(() => [] as string[]);
    const onDisk = BASELINE_UP.filter((name) => names.includes(name));
    const emitted = entries
      .filter((entry) => dirname(join(root, entry.filename)) === dir)
      .map((entry) => basename(entry.filename))
      .filter(isBaselineUp);
    if (onDisk.length !== 1 || emitted.length !== 1 || onDisk[0] === emitted[0]) continue;
    const from = emitted[0];
    const keep = onDisk[0];
    renameByDir.set(
      dir,
      new Map([
        [from, keep],
        [downOf(from), downOf(keep)],
      ]),
    );
  }
  return entries.map((entry) => {
    const name = renameByDir.get(dirname(join(root, entry.filename)))?.get(basename(entry.filename));
    return name === undefined ? entry : { ...entry, filename: entry.filename.replace(/[^/]+$/, name) };
  });
};

const pruneStaleCustomMigrations = async (
  fs: Fs,
  root: string,
  entries: { filename: string }[],
): Promise<void> => {
  const dirs = new Set(
    entries.map((entry) => dirname(join(root, entry.filename))).filter((dir) => basename(dir) === "migrations"),
  );
  const keep = new Set(entries.map((entry) => basename(entry.filename)));
  for (const dir of dirs) {
    const names = await fs.readdir(dir);
    for (const name of names) {
      if (STALE_CUSTOM.test(name) || (GENERATED_MIGRATION.test(name) && !keep.has(name))) await fs.unlink(join(dir, name));
    }
  }
};

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
  const generated = await generateSql(
    {
      reader,
      settings,
    },
    sqlDialectsFromEnv(await destEnv(deps.fs, flags.cwd)),
  );
  const root = at(flags.cwd, command.migrationsRoot);
  const entries = await preserveBaselineNames(deps.fs, root, generated);
  if (flags.dryRun) {
    for (const entry of entries) deps.log(join(root, entry.filename));
    return;
  }
  for (const path of await writeEntries(deps.fs, root, entries)) deps.log(path);
  await pruneStaleCustomMigrations(deps.fs, root, entries);
};
