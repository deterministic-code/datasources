import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const { parse } = require("../typescript/node_modules/yaml/dist/index.js");
const root = join(dirname(fileURLToPath(import.meta.url)), "..");

const columnSql = (column) =>
  `${column.name} ${column.type}${column.default == null ? "" : ` DEFAULT ${column.default}`}${column.notNull ? " NOT NULL" : ""}`;

const constraintSql = (constraint) => {
  const columns = (constraint.columns ?? []).join(", ");
  if (constraint.kind === "primaryKey") return `CONSTRAINT ${constraint.name} PRIMARY KEY (${columns})`;
  if (constraint.kind === "unique") return `CONSTRAINT ${constraint.name} UNIQUE (${columns})`;
  if (constraint.kind === "check") return `CONSTRAINT ${constraint.name} CHECK (${constraint.expression})`;
  if (constraint.kind === "foreignKey") {
    const references = (constraint.referencesColumns ?? []).join(", ");
    return `CONSTRAINT ${constraint.name} FOREIGN KEY (${columns}) REFERENCES ${constraint.references} (${references})`;
  }
  throw new Error(`${constraint.name}: empty constraint`);
};

const tableSql = (table, fragments) => {
  const specs = [
    ...(table.use ?? []).flatMap((name) => fragments[name] ?? (() => { throw new Error(`unknown fragment ${name}`); })()),
    ...(table.columns ?? []),
  ];
  const keys = specs.filter((column) => column.primaryKey).map((column) => column.name);
  const constraints = [
    ...(keys.length ? [`CONSTRAINT pk_${table.name} PRIMARY KEY (${keys.join(", ")})`] : []),
    ...(table.constraints ?? []).map(constraintSql),
  ];
  const parts = constraints.length ? [...specs.map(columnSql), ...constraints] : specs.map(columnSql);
  return `CREATE TABLE ${table.name} (${parts.join(", ")})`;
};

let catalog;

const file = () => (catalog ??= parse(readFileSync(join(root, "specs", "schemas.yaml"), "utf8")));

export const schemaNames = () => Object.keys(file().schemas);

export const schemaText = (name) => {
  const tables = file().schemas[name];
  if (!tables) throw new Error(`unknown schema ${name}`);
  return tables.map((table) => tableSql(table, file().fragments)).join(";\n");
};

export const catalogSchemas = () => schemaNames().map((name) => ({ name, sql: schemaText(name) }));
