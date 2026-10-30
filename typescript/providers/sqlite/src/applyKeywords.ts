const KEYWORDS = new Set([
  "AFTER", "ALTER", "AND", "AS", "AT", "AUTOINCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BLOB",
  "BOOLEAN", "BY", "CASCADE", "CAST", "CONSTRAINT", "CREATE", "DECIMAL", "DEFAULT", "DELETE",
  "DROP", "EACH", "ELSE", "END", "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM",
  "GENERATED", "IF", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY",
  "NOT", "NULL", "NUMERIC", "ON", "OR", "ORDER", "PRIMARY", "REAL", "REFERENCES", "RETURN",
  "RETURNING", "ROW", "SELECT", "SET", "TABLE", "TEXT", "THEN", "TRIGGER", "TRUE", "UNIQUE",
  "UPDATE", "VALUES", "VARCHAR", "WHEN", "WHERE",
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
