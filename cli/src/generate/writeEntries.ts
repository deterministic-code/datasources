import { join } from "node:path";
import { stripAttributes, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import type { Fs } from "../io.ts";

const merge = (existing: string, entry: Extract<GenerateEntry, { kind: "patch" }>): string => {
  if (entry.section !== undefined) {
    const start = `// <${entry.section}>`;
    const end = `// </${entry.section}>`;
    const block = `${start}\n${entry.content}\n${end}\n`;
    const pattern = new RegExp(`${start}[\\s\\S]*?${end}\\n?`);
    return pattern.test(existing) ? existing.replace(pattern, block) : `${existing}${block}`;
  }
  if (existing.includes(entry.content)) return existing;
  if (entry.appendIfNotExists === "None") return existing;
  if (entry.appendIfNotExists === "Start") return `${entry.content}${existing}`;
  return `${existing}${entry.content}`;
};

const listFiles = async (fs: Fs, dir: string): Promise<string[]> => {
  const names = await fs.readdir(dir).catch(() => [] as string[]);
  const files: string[] = [];
  for (const name of names) {
    const path = join(dir, name);
    const file = await fs.readFile(path).then(() => true).catch(() => false);
    if (file) files.push(path);
    else files.push(...(await listFiles(fs, path)));
  }
  return files;
};

export const deleteStale = async (fs: Fs, root: string, keep: string[]): Promise<string[]> => {
  const kept = new Set(keep);
  const removed: string[] = [];
  for (const path of await listFiles(fs, root)) {
    if (kept.has(path)) continue;
    await fs.unlink(path);
    removed.push(path);
  }
  return removed;
};

export const writeEntries = async (fs: Fs, root: string, entries: GenerateEntry[]): Promise<string[]> => {
  const paths: string[] = [];
  for (const entry of stripAttributes(entries)) {
    const path = join(root, entry.filename);
    if (entry.kind === "content") {
      await fs.writeFile(path, entry.contents);
    } else {
      const existing = await fs.readFile(path).catch(() => "");
      await fs.writeFile(path, merge(existing, entry));
    }
    paths.push(path);
  }
  return paths;
};
