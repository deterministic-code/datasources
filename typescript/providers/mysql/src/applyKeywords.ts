const KEYWORDS = new Set([
  "AFTER", "ALTER", "AND", "AS", "AUTO_INCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BIGINT",
  "BLOB", "BOOLEAN", "BY", "CASCADE", "CAST", "CHAR", "CONSTRAINT", "CREATE", "DATETIME",
  "DECIMAL", "DECLARE", "DEFAULT", "DELETE", "DOUBLE", "DROP", "EACH", "ELSE", "END",
  "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM", "GENERATED", "IF", "IN", "INDEX",
  "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY", "LAST_INSERT_ID", "LONGBLOB",
  "LONGTEXT", "NOT", "NOW", "NULL", "NUMERIC", "ON", "OR", "ORDER", "PRIMARY", "PROCEDURE",
  "REFERENCES", "REPLACE", "RETURN", "ROW", "ROW_COUNT", "SELECT", "SET", "SMALLINT", "TABLE",
  "TEXT", "THEN", "TIMESTAMP", "TINYINT", "TRIGGER", "TRUE", "UNIQUE", "UNSIGNED", "UPDATE",
  "UTC_TIMESTAMP", "VALUES", "VARBINARY", "VARCHAR", "WHEN", "WHERE",
]);

const TOKEN =
  /('(?:''|[^'])*')|(`(?:``|[^`])*`)|("(?:""|[^"])*")|(--[^\n]*)|(\/\*[\s\S]*?\*\/)|(?<word>[A-Za-z_][A-Za-z0-9_]*)/g;

export const applyKeywords = (
  sql: string,
  keyword: (word: string) => string,
): string =>
  sql.replace(TOKEN, (match, _s, _b, _d, _lc, _bc, word) =>
    word !== undefined && KEYWORDS.has(word.toUpperCase()) ? keyword(word) : match,
  );
