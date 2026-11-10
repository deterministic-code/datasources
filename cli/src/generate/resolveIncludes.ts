import { basename, dirname, join } from "node:path";
import { parse, stringify } from "yaml";
import {
  resolveIncludes as foldIncludes,
  resolveSeedIncludes as foldSeedIncludes,
} from "@deterministic-code/deterministic-generators-common/includes";
import type { Fs } from "../io.ts";

type Include = { file?: string; types?: { filter?: string } };
type Doc = { includes?: Include[] };

const asDoc = (text: string): Doc | undefined => {
  const parsed = parse(text);
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) return undefined;
  return parsed as Doc;
};

const isFile = (entry: Include): entry is Include & { file: string } =>
  typeof entry.file === "string" && entry.file.length > 0;

const foldOf = (path: string) =>
  basename(path) === "datasource_seeds.yaml" ? foldSeedIncludes : foldIncludes;

const emptyInclude = (path: string): string =>
  basename(path) === "datasource_seeds.yaml" ? "version: 1.0.0\nseeds: []\n" : "types: []\n";

export const resolveIncludes = async (fs: Fs, path: string, text: string): Promise<string> => {
  const doc = asDoc(text);
  const includes = Array.isArray(doc?.includes) ? doc.includes : [];
  const files = includes.filter(isFile);
  if (doc === undefined || files.length === 0) return text;
  const { yaml } = await foldOf(path)(
    stringify({ ...doc, includes: files }),
    async (include, ctx) => {
      const root = ctx.baseDir!;
      const file = include.file ?? "";
      if (file.length === 0) return { text: emptyInclude(path), key: join(root, "#"), ctx: { baseDir: root } };
      const child = join(root, file);
      return { text: await fs.readFile(child), key: child, ctx: { baseDir: dirname(child) } };
    },
    { baseDir: dirname(path) },
  );
  const filters = includes.filter((entry) => !isFile(entry));
  return filters.length === 0 ? yaml : stringify({ ...asDoc(yaml), includes: filters });
};
