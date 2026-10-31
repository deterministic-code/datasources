import { readFile } from "node:fs/promises";

const resource = (name: string): Promise<string> =>
  readFile(new URL(`./rust/${name}`, import.meta.url), "utf8");

const [repository, crud, registryTmpl, modTmpl, crateModTmpl, schemaObjectsTmpl] = await Promise.all([
  resource("repository.rs.tmpl"),
  resource("crud.rs.tmpl"),
  resource("registry.rs.tmpl"),
  resource("mod.rs.tmpl"),
  resource("crate_mod.rs.tmpl"),
  resource("schema_objects.rs.tmpl"),
]);

export const repositoryTmpl = repository.replaceAll("{{>crud}}", crud);
export { registryTmpl, modTmpl, crateModTmpl, schemaObjectsTmpl };
