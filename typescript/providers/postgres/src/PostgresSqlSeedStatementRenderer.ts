import type { SqlSeedColumn, SqlSeedStatement } from "@deterministic-code/datasources";

const quote = (ident: string): string => `"${ident}"`;

const quoteString = (value: unknown): string => `'${String(value).replace(/'/g, "''")}'`;

const truthy = (value: unknown): boolean =>
  value === true || value === "true" || value === "True" || value === 1;

const numeric = new Set([
  "integer",
  "unsignedinteger",
  "smallinteger",
  "unsignedsmallinteger",
  "biginteger",
  "unsignedbiginteger",
  "reference",
  "float",
  "decimal",
]);

const defaultSql = (type: string, def: unknown): string => {
  const text = String(def);
  if (type === "datetime" && text === "UtcNow") return "(NOW() AT TIME ZONE 'UTC')";
  if (type === "datetime" && text === "Now") return "LOCALTIMESTAMP";
  if (type === "uuid" && text === "NewId") return "gen_random_uuid()";
  if (type === "boolean") return text === "true" || text === "True" ? "TRUE" : "FALSE";
  return quoteString(text);
};

const literal = (column: SqlSeedColumn): string => {
  if (column.value == null)
    return column.field.defaultValue != null ? defaultSql(column.field.type, column.field.defaultValue) : "NULL";
  if (column.field.type === "boolean") return truthy(column.value) ? "TRUE" : "FALSE";
  if (numeric.has(column.field.type)) return String(column.value);
  if (column.field.type === "binary") return `'\\x${column.value}'`;
  return quoteString(column.value);
};

export class PostgresSqlSeedStatementRenderer {
  render(statement: SqlSeedStatement): string {
    const table = quote(statement.table);
    if (statement.kind === "delete") {
      const key = statement.columns[0]!;
      return `DELETE FROM ${table} WHERE ${quote(key.column)} = ${literal(key)}`;
    }
    const cols = statement.columns.map((c) => quote(c.column)).join(", ");
    const values = statement.columns.map(literal).join(", ");
    const insert = `INSERT INTO ${table} (${cols}) VALUES (${values})`;
    if (!statement.identity || statement.idColumn == null) return insert;
    const id = statement.idColumn;
    return `${insert};\nSELECT setval(pg_get_serial_sequence('${statement.table}', '${id}'), (SELECT MAX(${quote(id)}) FROM ${table}))`;
  }
}
