import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { execute, type IConnection, type IDriver, type IStatement } from "@deterministic-code/datasources";
import { parse } from "yaml";
import { createCasing } from "./code-generators/src/sql/common/default-casing.ts";
import { register } from "./code-generators/src/sql/common/sql-dialect.ts";
import { SqlMapping } from "./code-generators/src/sql/common/sql-mapping.ts";
import { buildLiveTables, sqlTablesFrom } from "./code-generators/src/sql/common/sql-schema.ts";
import { createIndexStatementOf, createTableStatementOf } from "./code-generators/src/sql/statements.ts";
import { specsDir } from "./schema-spec.ts";

type SettingNode = string | number | boolean | null | SettingNode[] | { [key: string]: SettingNode };

const files = (): Record<string, string> => {
  const dir = join(specsDir(), "contacts");
  return {
    "types.yaml": readFileSync(join(dir, "types.yaml"), "utf8"),
    "datasource.yaml": readFileSync(join(dir, "datasource.yaml"), "utf8"),
  };
};

const flatten = (node: SettingNode, prefix: string, out: Record<string, string>): void => {
  if (Array.isArray(node)) {
    if (prefix) out[prefix] = node.map((item) => String(item)).join(",");
    return;
  }
  if (node !== null && typeof node === "object") {
    for (const [key, value] of Object.entries(node)) flatten(value, prefix ? `${prefix}.${key}` : key, out);
    return;
  }
  if (prefix) out[prefix] = String(node);
};

const settingsOf = (yaml: string): Record<string, string> => {
  const parsed = parse(yaml) as SettingNode;
  const out: Record<string, string> = {};
  if (parsed !== null && typeof parsed === "object" && !Array.isArray(parsed) && parsed.settings !== undefined)
    flatten(parsed.settings, "", out);
  return out;
};

const contactStatements = async (dialect: string): Promise<IStatement[]> => {
  const loaded = files();
  const settings = settingsOf(loaded["datasource.yaml"]);
  const spec = await DeterministicParser(memoryReader(loaded)).parse(settings);
  const types = sqlTablesFrom(spec);
  const mapping = new SqlMapping(createCasing(settings), types);
  return buildLiveTables(types, mapping).flatMap((table) => [
    createTableStatementOf(table, mapping),
    ...table.indexes.map((index) => createIndexStatementOf(mapping, table, index)),
  ]);
};

export const applyContacts = async (
  connection: IConnection,
  dialect: string,
  driver: IDriver,
): Promise<void> => {
  register(dialect, driver);
  const command = connection.createCommand();
  for (const statement of await contactStatements(dialect)) await execute(command, statement);
};
