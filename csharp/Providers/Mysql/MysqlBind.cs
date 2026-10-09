using System.Text.RegularExpressions;

namespace Datasources.Mysql;

internal static class MysqlBind
{
    static readonly Regex Name = new(@"\$([A-Za-z_][A-Za-z0-9_]*)", RegexOptions.Compiled);
    static readonly Regex Token = new(@"\$[A-Za-z_][A-Za-z0-9_]*|'[^']*'|""[^""]*""|`[^`]*`|\b[A-Za-z_][A-Za-z0-9_]*\b", RegexOptions.Compiled);

    public static string WithoutReturning(string sql) =>
        Regex.Replace(sql, @"\s+RETURNING\b[^;]*$", "", RegexOptions.IgnoreCase);

    static string QuoteReserved(string sql) =>
        Token.Replace(sql, match =>
        {
            var token = match.Value;
            var skip = token.StartsWith('$') || token.StartsWith('\'') || token.StartsWith('"') || token.StartsWith('`');
            var reserved = !skip && token.Equals("key", StringComparison.OrdinalIgnoreCase)
                && !Regex.IsMatch(sql[..match.Index], @"\b(?:PRIMARY|FOREIGN|UNIQUE)\s+$", RegexOptions.IgnoreCase);
            var mixed = !skip && token.Any(char.IsUpper) && token.Any(char.IsLower);
            return reserved || mixed ? $"`{token}`" : token;
        });

    public static string ToMysql(string sql) => Name.Replace(QuoteReserved(WithoutReturning(sql)), "@$1");

    public static string IsolationSql(IsolationLevel isolation) => isolation switch
    {
        IsolationLevel.ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel.ReadCommitted => "READ COMMITTED",
        _ => "REPEATABLE READ"
    };
}
