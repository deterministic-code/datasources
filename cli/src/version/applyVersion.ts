import { isAbsolute, join } from "node:path";
import type { Flags, VersionCommand } from "../parse.ts";
import type { Deps } from "../io.ts";
import { VersionSnapshot } from "@deterministic-code/deterministic-generators-common/parser";
import type { Json } from "./delta.ts";
import { parseYaml, render } from "./render.ts";

const versionDir = /^version-\d{8}T\d{6}Z-(\d{3})$/;

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

export const applyVersion = async (
  command: VersionCommand,
  flags: Flags,
  deps: Deps,
  now = () => new Date(),
): Promise<void> => {
  const root = at(flags.cwd, command.input);
  const names = await deps.fs.readdir(root);
  const versions = names.filter((name) => versionDir.test(name)).sort();
  const diffs = (
    await Promise.all(
      versions.map(async (dir) => {
        const listed = await deps.fs.readdir(join(root, dir));
        if (!listed.includes("diff.yaml")) return undefined;
        const text = await deps.fs.readFile(join(root, dir, "diff.yaml"));
        return text === "" ? undefined : parseYaml(text);
      }),
    )
  ).filter((diff) => diff !== undefined);
  const before = VersionSnapshot.combine(diffs).documents as { [path: string]: Json };
  const current = names.filter((name) => name.endsWith(".yaml")).sort();
  const paths = [...new Set([...Object.keys(before), ...current])];
  const yaml = render(
    await Promise.all(
      paths.map(async (path) => ({
        path,
        before: before[path],
        after: current.includes(path) ? parseYaml(await deps.fs.readFile(join(root, path))) : undefined,
      })),
    ),
  );
  if (yaml === "") {
    deps.log("unchanged");
    return;
  }
  const out = join(root, `version-${stamp(now())}-${pad(nextSeq(versions))}`, "diff.yaml");
  if (!flags.dryRun) await deps.fs.writeFile(out, yaml);
  deps.log(out);
};

const nextSeq = (versions: string[]): number => {
  const next =
    versions.reduce((max, name) => Math.max(max, Number(name.match(versionDir)![1])), 0) + 1;
  return next <= 999 ? next : (() => { throw new Error("version sequence exceeds 999"); })();
};

const pad = (n: number): string => String(n).padStart(3, "0");

const stamp = (date: Date): string =>
  date.toISOString().replace(/[-:]/g, "").replace(/\.\d{3}/, "");
