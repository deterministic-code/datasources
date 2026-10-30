using System.Text.RegularExpressions;

namespace Datasources.Mysql;

static class SqlKeywords
{
    static readonly HashSet<string> All =
    [
        "AFTER", "ALTER", "AND", "AS", "AUTO_INCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BIGINT",
        "BLOB", "BOOLEAN", "BY", "CASCADE", "CAST", "CHAR", "CONSTRAINT", "CREATE", "DATETIME",
        "DECIMAL", "DECLARE", "DEFAULT", "DELETE", "DOUBLE", "DROP", "EACH", "ELSE", "END",
        "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN", "FROM", "GENERATED", "IF", "IN", "INDEX",
        "INNER", "INSERT", "INT", "INTEGER", "INTO", "JOIN", "KEY", "LAST_INSERT_ID", "LONGBLOB",
        "LONGTEXT", "NOT", "NOW", "NULL", "NUMERIC", "ON", "OR", "ORDER", "PRIMARY", "PROCEDURE",
        "REFERENCES", "REPLACE", "RETURN", "ROW", "ROW_COUNT", "SELECT", "SET", "SMALLINT", "TABLE",
        "TEXT", "THEN", "TIMESTAMP", "TINYINT", "TRIGGER", "TRUE", "UNIQUE", "UNSIGNED", "UPDATE",
        "UTC_TIMESTAMP", "VALUES", "VARBINARY", "VARCHAR", "WHEN", "WHERE",
    ];

    static readonly Regex Token = new(
        @"('(?:''|[^'])*')|(`(?:``|[^`])*`)|(""(?:""""|[^""])*"")|(--[^\n]*)|(/\*[\s\S]*?\*/)|(?<word>[A-Za-z_][A-Za-z0-9_]*)",
        RegexOptions.Compiled);

    internal static string Apply(string sql, Func<string, string> keyword) =>
        Token.Replace(sql, match =>
            match.Groups["word"].Success && All.Contains(match.Groups["word"].Value.ToUpperInvariant())
                ? keyword(match.Groups["word"].Value)
                : match.Value);
}
