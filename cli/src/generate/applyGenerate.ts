import { isAbsolute, join, relative } from "node:path";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import type { Flags, GenerateCommand } from "../parse.ts";
import type { Deps } from "../io.ts";
import { destEnv } from "./destEnv.ts";
import { loadSpec } from "./loadSpec.ts";
import { providerByName } from "./providers.ts";
import { requireKnownReferenceTypes } from "./requireKnownReferenceTypes.ts";
import { withStoredProcedures } from "./storedProcedures.ts";
import { deleteStale, writeEntries } from "./writeEntries.ts";
import { overlayCasingFlags } from "./casing.ts";
import { overlaySqlFlags } from "./sqlFlags.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

export const applyGenerate = async (command: GenerateCommand, flags: Flags, deps: Deps): Promise<void> => {
  const provider = providerByName(command.provider);
  const loaded = await loadSpec(deps.fs, at(flags.cwd, command.input));
  const settings = overlayCasingFlags(
    overlaySqlFlags(withStoredProcedures(loaded.settings, command.storedProcedures), command),
    command.dialect,
    command,
  );
  const reader = memoryReader(loaded.files);
  requireKnownReferenceTypes(await DeterministicParser(reader).parse(settings));
  const request = {
    dialect: command.dialect,
    reader,
    settings,
    env: await destEnv(deps.fs, flags.cwd),
  };
  const entries = await {
    all: async () => [
      ...(await provider.generateTypes(request)),
      ...(await provider.generateRepositories(request)),
    ],
    types: () => provider.generateTypes(request),
    repositories: () => provider.generateRepositories(request),
    datasource: () => provider.generateDatasource(request),
  }[command.target]();
  const root = at(flags.cwd, command.output);
  const written = await writeEntries(deps.fs, root, entries);
  for (const path of written) deps.log(path);
  if (command.delete) for (const path of await deleteStale(deps.fs, root, written)) deps.log(path);
  if (command.dialect !== "rust" || command.target === "datasource") return;
  const dest = join(flags.cwd, "src", "generated.rs");
  const rustMod = join(at(flags.cwd, command.output), "mod.rs");
  const path = relative(join(flags.cwd, "src"), rustMod).replaceAll("\\", "/");
  await deps.fs.writeFile(
    dest,
    `#[path = "${path}"]\nmod generated_mod;\npub use generated_mod::*;\npub use generated_mod::Repositories;\n`,
  );
  deps.log(dest);
};
