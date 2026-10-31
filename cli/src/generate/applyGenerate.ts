import { isAbsolute, join, relative } from "node:path";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import type { Flags, GenerateCommand } from "../parse.ts";
import type { Deps } from "../io.ts";
import { destEnv } from "./destEnv.ts";
import { loadSpec } from "./loadSpec.ts";
import { providerByName } from "./providers.ts";
import { withStoredProcedures } from "./storedProcedures.ts";
import { writeEntries } from "./writeEntries.ts";
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
  const request = {
    dialect: command.dialect,
    reader: memoryReader(loaded.files),
    settings,
    env: await destEnv(deps.fs, flags.cwd),
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
  if (command.dialect !== "rust" || command.target === "datasource") return;
  const dest = join(flags.cwd, "src", "generated.rs");
  const rustMod = join(at(flags.cwd, command.output), "rust", "mod.rs");
  const path = relative(join(flags.cwd, "src"), rustMod).replaceAll("\\", "/");
  await deps.fs.writeFile(
    dest,
    `#[path = "${path}"]\nmod generated_mod;\npub use generated_mod::*;\npub use generated_mod::Repositories;\n`,
  );
  deps.log(dest);
};
