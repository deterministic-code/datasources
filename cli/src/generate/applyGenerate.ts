import { isAbsolute, join } from "node:path";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import type { Flags, GenerateCommand } from "../parse.ts";
import type { Deps } from "../run.ts";
import { loadSpec } from "./loadSpec.ts";
import { providerByName } from "./providers.ts";
import { withStoredProcedures } from "./storedProcedures.ts";
import { writeEntries } from "./writeEntries.ts";
import { overlayCasingFlags } from "./casing.ts";
import { overlaySqlFlags } from "./sqlFlags.ts";
import { wireRegister } from "../wireRegister.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

export const applyGenerate = async (command: GenerateCommand, flags: Flags, deps: Deps): Promise<void> => {
  const provider = providerByName(command.provider);
  const loaded = await loadSpec(deps.fs, at(flags.cwd, command.input));
  const settings = overlayCasingFlags(
    overlaySqlFlags(withStoredProcedures(loaded.settings, command.storedProcedures), command),
    command.dialect,
    command,
  );
  const request = {
    dialect: command.dialect,
    reader: memoryReader(loaded.files),
    settings,
  };
  const entries = await {
    all: async () => [
      ...(await provider.generateTypes(request)),
      ...(await provider.generateRepositories(request)),
      ...(await provider.generateDatasource(request)),
    ],
    types: () => provider.generateTypes(request),
    repositories: () => provider.generateRepositories(request),
    datasource: () => provider.generateDatasource(request),
  }[command.target]();
  for (const path of await writeEntries(deps.fs, at(flags.cwd, command.output), entries)) deps.log(path);
  if (command.target === "all" || command.target === "repositories")
    await wireRegister(flags.cwd, command.output, command.dialect, flags.dryRun, deps, settings);
};
