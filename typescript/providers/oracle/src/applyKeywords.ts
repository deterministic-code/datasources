const KEYWORDS = new Set([
  "AFTER", "ALTER", "AND", "AS", "AT", "BEFORE", "BEGIN", "BETWEEN", "BINARY_DOUBLE", "BLOB",
  "BY", "CASCADE", "CAST", "CHAR", "CLOB", "CONSTRAINT", "CONSTRAINTS", "CREATE", "DECIMAL",
  "DECLARE", "DEFAULT", "DELETE", "DROP", "EACH", "ELSE", "END", "EXCEPTION", "EXECUTE",
  "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM", "FUNCTION", "IF", "IMMEDIATE", "IN",
  "INDEX", "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY", "LANGUAGE",
  "LOCALTIMESTAMP", "NCHAR", "NOT", "NULL", "NUMBER", "NUMERIC", "OLD", "ON", "OR", "ORDER",
  "OTHERS", "PRECISION", "PRIMARY", "PROCEDURE", "RAISE", "RAW", "REFERENCES", "REPLACE",
  "RETURN", "RETURNING", "ROW", "SELECT", "SET", "SMALLINT", "SQLCODE", "SYS_EXTRACT_UTC",
  "SYS_GUID", "SYSTIMESTAMP", "TABLE", "THEN", "TIMESTAMP", "TRIGGER", "TRUE", "UNIQUE",
  "UPDATE", "VALUES", "VARCHAR", "VARCHAR2", "WHEN", "WHERE", "ZONE",
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
