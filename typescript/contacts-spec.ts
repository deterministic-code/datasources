import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { execute, type IConnection, type IDriver, type IStatement } from "@deterministic-code/datasources";
import { createCasing } from "./code-generators/src/sql/common/default-casing.ts";
import { schemaDiff } from "./code-generators/src/sql/common/schema-diff.ts";
import { register } from "./code-generators/src/sql/common/sql-dialect.ts";
import { buildLiveTables, sqlTablesFrom } from "./code-generators/src/sql/common/sql-schema.ts";
import { specsDir } from "./schema-spec.ts";

const settings = { "datasource.pluralize_datatable_names": "false" };

const files = (): Record<string, string> => {
  const dir = join(specsDir(), "contacts");
  return {
    "types.yaml": readFileSync(join(dir, "types.yaml"), "utf8"),
    "datasource.yaml": readFileSync(join(dir, "datasource.yaml"), "utf8"),
  };
};

const contactStatements = async (dialect: string): Promise<IStatement[]> => {
  const spec = await DeterministicParser(memoryReader(files())).parse(settings);
  const naming = createCasing(settings).naming;
  const tables = buildLiveTables(sqlTablesFrom(spec), naming);
  return schemaDiff(dialect, { tables: [], naming }, { tables, naming });
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
