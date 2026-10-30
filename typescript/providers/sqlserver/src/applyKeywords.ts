const KEYWORDS = new Set([
  "AFTER", "ALTER", "AND", "AS", "AT", "BEGIN", "BETWEEN", "BIGINT", "BIT", "BY", "CASCADE",
  "CAST", "CHAR", "CONSTRAINT", "CREATE", "DATETIME", "DATETIME2", "DECIMAL", "DECLARE",
  "DEFAULT", "DELETE", "DROP", "EACH", "ELSE", "END", "EXECUTE", "EXISTS", "FALSE", "FLOAT",
  "FOR", "FOREIGN", "FROM", "GET", "GETDATE", "GETUTCDATE", "GO", "IDENTITY", "IDENTITY_INSERT",
  "IF", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY", "MAX",
  "NCHAR", "NEWID", "NOCOUNT", "NOT", "NULL", "NVARCHAR", "OFF", "ON", "OR", "ORDER",
  "PRECISION", "PRIMARY", "PROCEDURE", "REFERENCES", "RETURN", "ROW", "ROWCOUNT",
  "SCOPE_IDENTITY", "SELECT", "SET", "SMALLINT", "TABLE", "TEXT", "THEN", "TIME", "TIMESTAMP",
  "TINYINT", "TRIGGER", "TRUE", "UNIQUE", "UNIQUEIDENTIFIER", "UPDATE", "VALUES", "VARBINARY",
  "VARCHAR", "WHEN", "WHERE",
]);

const TOKEN =
  /('(?:''|[^'])*')|(\[[^\]]*\])|(--[^\n]*)|(\/\*[\s\S]*?\*\/)|(?<word>[A-Za-z_][A-Za-z0-9_]*)/g;

export const applyKeywords = (
  sql: string,
  keyword: (word: string) => string,
): string =>
  sql.replace(TOKEN, (match, _s, _br, _lc, _bc, word) =>
    word !== undefined && KEYWORDS.has(word.toUpperCase()) ? keyword(word) : match,
  );
