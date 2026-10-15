using System.Text.RegularExpressions;

namespace Datasources.CodeGenerators;

public static class SqlKeywords
{
    public static readonly HashSet<string> All =
    [
        "AFTER", "ALTER", "AND", "AS", "AT", "AUTOINCREMENT", "AUTO_INCREMENT", "BEFORE", "BEGIN",
        "BETWEEN", "BIGINT", "BIGSERIAL", "BINARY_DOUBLE", "BIT", "BLOB", "BOOLEAN", "BY", "BYTEA",
        "CASCADE", "CAST", "CHAR", "CLOB", "CONSTRAINT", "CONSTRAINTS", "CREATE", "DATETIME",
        "DATETIME2", "DECIMAL", "DECLARE", "DEFAULT", "DELETE", "DIAGNOSTICS", "DOUBLE", "DROP",
        "EACH", "ELSE", "END", "EXCEPTION", "EXECUTE", "EXISTS", "FALSE", "FLOAT", "FOR", "FOREIGN",
        "FROM", "FUNCTION", "GENERATED", "GET", "GETDATE", "GETUTCDATE", "GO", "IDENTITY",
        "IDENTITY_INSERT", "IF", "IMMEDIATE", "IN", "INDEX", "INNER", "INSERT", "INT", "INTEGER",
        "INTO", "JOIN", "KEY", "LANGUAGE", "LAST_INSERT_ID", "LOCALTIMESTAMP", "LONGBLOB",
        "LONGTEXT", "MAX", "NCHAR", "NEW", "NEWID", "NOCOUNT", "NOT", "NOW", "NULL", "NUMBER",
        "NUMERIC", "NVARCHAR", "OFF", "OLD", "ON", "OR", "ORDER", "OTHERS", "PRECISION", "PRIMARY",
        "PROCEDURE", "RAISE", "RAW", "REAL", "REFERENCES", "REPLACE", "RETURN", "RETURNING",
        "RETURNS", "ROW", "ROWCOUNT", "ROW_COUNT", "SCOPE_IDENTITY", "SELECT", "SERIAL", "SET",
        "SETOF", "SMALLINT", "SQLCODE", "SYS_EXTRACT_UTC", "SYS_GUID", "SYSTIMESTAMP", "TABLE",
        "TEXT", "THEN", "TIME", "TIMESTAMP", "TIMESTAMPTZ", "TINYINT", "TRIGGER", "TRUE", "UNIQUE",
        "UNIQUEIDENTIFIER", "UNSIGNED", "UPDATE", "UTC_TIMESTAMP", "UUID", "VALUES", "VARBINARY",
        "VARCHAR", "VARCHAR2", "WHEN", "WHERE", "ZONE",
    ];

    static readonly Regex Token = new(
        @"('(?:''|[^'])*')|(""(?:""""|[^""])*"")|(`(?:``|[^`])*`)|(\[[^\]]*\])|(--[^\n]*)|(/\*[\s\S]*?\*/)|([A-Za-z_][A-Za-z0-9_]*)",
        RegexOptions.Compiled);

    public static string Apply(string sql, Func<string, string> keyword) =>
        Token.Replace(sql, match =>
            match.Groups[7].Success && All.Contains(match.Groups[7].Value.ToUpperInvariant())
                ? keyword(match.Groups[7].Value)
                : match.Value);
}
