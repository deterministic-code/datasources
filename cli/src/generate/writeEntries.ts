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
