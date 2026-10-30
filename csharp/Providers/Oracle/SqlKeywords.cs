using System.Text.RegularExpressions;

namespace Datasources.Oracle;

static class SqlKeywords
{
    static readonly HashSet<string> All =
    [
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
    ];

    static readonly Regex Token = new(
        @"('(?:''|[^'])*')|(""(?:""""|[^""])*"")|(--[^\n]*)|(/\*[\s\S]*?\*/)|(?<word>[A-Za-z_][A-Za-z0-9_]*)",
        RegexOptions.Compiled);

    internal static string Apply(string sql, Func<string, string> keyword) =>
        Token.Replace(sql, match =>
            match.Groups["word"].Success && All.Contains(match.Groups["word"].Value.ToUpperInvariant())
                ? keyword(match.Groups["word"].Value)
                : match.Value);
}
