import { readFile } from "node:fs/promises";

const resource = (name: string): Promise<string> =>
  readFile(new URL(`./csharp/${name}`, import.meta.url), "utf8");

export const [repositoryTmpl, registryTmpl] = await Promise.all([
  resource("repository.cs.tmpl"),
  resource("registry.cs.tmpl"),
]);
