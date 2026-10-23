import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { execute, type IConnection, type IDriver, type IStatement } from "@deterministic-code/datasources";
import { createCasing } from "./code-generators/src/sql/common/default-casing.ts";
import { register } from "./code-generators/src/sql/common/sql-dialect.ts";
import { SqlMapping } from "./code-generators/src/sql/common/sql-mapping.ts";
import { buildLiveTables, sqlTablesFrom } from "./code-generators/src/sql/common/sql-schema.ts";
import { createIndexStatementOf, createTableStatementOf } from "./code-generators/src/sql/statements.ts";
import { repoRoot } from "./schema-spec.ts";

const settings = { "datasource.pluralize_datatable_names": "false" };

const files = (): Record<string, string> => {
  const dir = join(repoRoot(), "specs", "contacts");
  return {
    "types.yaml": readFileSync(join(dir, "types.yaml"), "utf8"),
    "datasource.yaml": readFileSync(join(dir, "datasource.yaml"), "utf8"),
  };
};

export const contactStatements = async (dialect: string): Promise<IStatement[]> => {
  const spec = await DeterministicParser(memoryReader(files())).parse(settings);
  const types = sqlTablesFrom(spec);
  const mapping = new SqlMapping(createCasing(settings), types);
  return buildLiveTables(types, mapping).flatMap((table) => [
    createTableStatementOf(dialect, table, mapping),
    ...table.indexes.map((index) => createIndexStatementOf(dialect, mapping, table, index)),
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
