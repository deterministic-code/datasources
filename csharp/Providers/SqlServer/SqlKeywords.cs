using System.Text.RegularExpressions;

namespace Datasources.SqlServer;

static class SqlKeywords
{
    static readonly HashSet<string> All =
    [
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
    ];

    static readonly Regex Token = new(
        @"('(?:''|[^'])*')|(\[[^\]]*\])|(--[^\n]*)|(/\*[\s\S]*?\*/)|(?<word>[A-Za-z_][A-Za-z0-9_]*)",
        RegexOptions.Compiled);

    internal static string Apply(string sql, Func<string, string> keyword) =>
        Token.Replace(sql, match =>
            match.Groups["word"].Success && All.Contains(match.Groups["word"].Value.ToUpperInvariant())
                ? keyword(match.Groups["word"].Value)
                : match.Value);
}
