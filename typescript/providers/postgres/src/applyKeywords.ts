const KEYWORDS = new Set([
  "AFTER", "ALTER", "AND", "AS", "AT", "BEFORE", "BEGIN", "BETWEEN", "BIGINT", "BIGSERIAL",
  "BOOLEAN", "BY", "BYTEA", "CASCADE", "CAST", "CHAR", "CONSTRAINT", "CREATE", "DECIMAL",
  "DECLARE", "DEFAULT", "DELETE", "DIAGNOSTICS", "DOUBLE", "DROP", "EACH", "ELSE", "END",
  "EXCEPTION", "EXECUTE", "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM", "FUNCTION",
  "GENERATED", "GET", "IF", "IMMEDIATE", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER",
  "INTO", "JOIN", "KEY", "LANGUAGE", "NEW", "NOT", "NOW", "NULL", "NUMERIC", "OLD", "ON", "OR",
  "ORDER", "OTHERS", "PRECISION", "PRIMARY", "PROCEDURE", "RAISE", "REAL", "REFERENCES",
  "REPLACE", "RETURN", "RETURNING", "RETURNS", "ROW", "ROW_COUNT", "SELECT", "SERIAL", "SET",
  "SETOF", "SMALLINT", "SQLCODE", "TABLE", "TEXT", "THEN", "TIME", "TIMESTAMP", "TIMESTAMPTZ",
  "TRIGGER", "TRUE", "UNIQUE", "UPDATE", "UUID", "VALUES", "VARCHAR", "WHEN", "WHERE", "ZONE",
]);

const TOKEN =
  /('(?:''|[^'])*')|("(?:""|[^"])*")|(--[^\n]*)|(\/\*[\s\S]*?\*\/)|(?<word>[A-Za-z_][A-Za-z0-9_]*)/g;

export const applyKeywords = (
  sql: string,
  keyword: (word: string) => string,
): string =>
  sql.replace(TOKEN, (match, _s, _d, _lc, _bc, word) =>
    word !== undefined && KEYWORDS.has(word.toUpperCase()) ? keyword(word) : match,
  );
