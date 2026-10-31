import { readFile } from "node:fs/promises";

const resource = (name: string): Promise<string> =>
  readFile(new URL(`./typescript/${name}`, import.meta.url), "utf8");

export const [repositoryTmpl, registryTmpl] = await Promise.all([
  resource("repository.ts.tmpl"),
  resource("registry.ts.tmpl"),
]);
