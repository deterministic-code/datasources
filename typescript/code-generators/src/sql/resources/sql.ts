import { readFile } from "node:fs/promises";

const resource = (rel: string): Promise<string> =>
  readFile(new URL(`../templates/create-sql/${rel}`, import.meta.url), "utf8");

export const [migrationUpTmpl, migrationDownTmpl] = await Promise.all([
  resource("migration-up.sql.tmpl"),
  resource("migration-down.sql.tmpl"),
]);
