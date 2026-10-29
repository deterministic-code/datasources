import { join } from "node:path";
import type { Fs } from "./io.ts";

const exclude = `  <PropertyGroup>
    <DefaultItemExcludes>$(DefaultItemExcludes);node_modules/**</DefaultItemExcludes>
  </PropertyGroup>
`;

const optional = async (fs: Fs, path: string): Promise<string | undefined> => {
  try {
    return await fs.readFile(path);
  } catch {
    return undefined;
  }
};

export const csharpBuildProps = (existing?: string): string | undefined => {
  if (existing?.includes("node_modules/**")) return undefined;
  return existing === undefined
    ? `<Project>\n${exclude}</Project>\n`
    : existing.replace(/<\/Project>\s*$/, `${exclude}</Project>\n`);
};

export const wireCsharpBuildProps = async (
  cwd: string,
  dryRun: boolean,
  deps: { fs: Fs; log: (line: string) => void },
): Promise<void> => {
  const path = join(cwd, "Directory.Build.props");
  const next = csharpBuildProps(await optional(deps.fs, path));
  if (next === undefined) return;
  if (dryRun) {
    deps.log(path);
    return;
  }
  await deps.fs.writeFile(path, next);
  deps.log(path);
};
