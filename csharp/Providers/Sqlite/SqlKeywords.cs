using System.Text.RegularExpressions;

namespace Datasources.Sqlite;

static class SqlKeywords
{
    static readonly HashSet<string> All =
    [
        "AFTER", "ALTER", "AND", "AS", "AT", "AUTOINCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BLOB",
        "BOOLEAN", "BY", "CASCADE", "CAST", "CONSTRAINT", "CREATE", "DECIMAL", "DEFAULT", "DELETE",
        "DROP", "EACH", "ELSE", "END", "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM",
        "GENERATED", "IF", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY",
        "NOT", "NULL", "NUMERIC", "ON", "OR", "ORDER", "PRIMARY", "REAL", "REFERENCES", "RETURN",
        "RETURNING", "ROW", "SELECT", "SET", "TABLE", "TEXT", "THEN", "TRIGGER", "TRUE", "UNIQUE",
        "UPDATE", "VALUES", "VARCHAR", "WHEN", "WHERE",
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
