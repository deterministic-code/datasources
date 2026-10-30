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
  if (type === "datetime" && text === "UtcNow") return "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))";
  if (type === "datetime" && text === "Now") return "(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))";
  if (type === "uuid" && text === "NewId")
    return "(lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6))))";
  if (type === "boolean") return text === "true" || text === "True" ? "1" : "0";
  return quoteString(text);
};

const literal = (column: SqlSeedColumn): string => {
  if (column.value == null)
    return column.field.defaultValue != null ? defaultSql(column.field.type, column.field.defaultValue) : "NULL";
  if (column.field.type === "boolean") return truthy(column.value) ? "1" : "0";
  if (numeric.has(column.field.type)) return String(column.value);
  if (column.field.type === "binary") return `X'${column.value}'`;
  return quoteString(column.value);
};

export class SqliteSqlSeedStatementRenderer {
  render(statement: SqlSeedStatement): string {
    const table = quote(statement.table);
    if (statement.kind === "delete") {
      const key = statement.columns[0]!;
      return `DELETE FROM ${table} WHERE ${quote(key.column)} = ${literal(key)}`;
    }
    const cols = statement.columns.map((c) => quote(c.column)).join(", ");
    const values = statement.columns.map(literal).join(", ");
    return `INSERT INTO ${table} (${cols}) VALUES (${values})`;
  }
}
