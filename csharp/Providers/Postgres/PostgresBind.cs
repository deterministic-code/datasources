using System.Text.RegularExpressions;

namespace Datasources.Postgres;

public static class PostgresBind
{
    static readonly Regex Name = new(@"\$([A-Za-z_][A-Za-z0-9_]*)", RegexOptions.Compiled);

    public static (string Sql, object?[] Values) ToPostgres(string sql, Row parameters)
    {
        var names = new List<string>();
        var text = Name.Replace(sql, match =>
        {
            var name = match.Groups[1].Value;
            var index = names.IndexOf(name);
            if (index < 0)
            {
                names.Add(name);
                index = names.Count - 1;
            }
            return $"${index + 1}";
        });
        return (text, names.Select(name => parameters.TryGetValue(name, out var value) ? value.ToObject() : null).ToArray());
    }

    public static string WithReturning(string sql) =>
        Regex.IsMatch(sql, @"^\s*insert\b", RegexOptions.IgnoreCase)
        && !Regex.IsMatch(sql, @"\breturning\b", RegexOptions.IgnoreCase)
            ? $"{sql.TrimEnd().TrimEnd(';')} RETURNING id"
            : sql;

    public static string IsolationSql(IsolationLevel isolation) => isolation switch
    {
        IsolationLevel.ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel.ReadCommitted => "READ COMMITTED",
        IsolationLevel.RepeatableRead => "REPEATABLE READ",
        _ => "SERIALIZABLE"
    };
}
