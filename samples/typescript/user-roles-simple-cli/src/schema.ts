import { type CreateTableStatement } from "@datasources/core";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { tables } from "../../../../typescript/schema-spec.ts";

const quote = (provider: string, id: string): string =>
  provider === "mysql" ? `\`${id.replaceAll("`", "``")}\`` : `"${id.replaceAll('"', '""')}"`;

export const ident = (provider: string, name: string): string => quote(provider, name);

export const qualified = (provider: string, name: string): string =>
  name.split(".").map((part) => quote(provider, part)).join(".");

export const flag = (provider: string, value: boolean): number | boolean =>
  provider === "sqlite" ? (value ? 1 : 0) : value;

export const schema = (provider: string): CreateTableStatement[] =>
  tables("user-roles-cli", (name) => ident(provider, name));

export const mysqlProcedures = (): string[] =>
  readFileSync(
    join(dirname(fileURLToPath(import.meta.url)), "../../../user-roles-simple-cli-shared/mysql-procedures.sql"),
    "utf8",
  )
    .split(/^---\s*$/m)
    .map((sql) => sql.trim())
    .filter(Boolean);
