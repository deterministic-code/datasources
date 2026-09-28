using System.Text.RegularExpressions;

namespace Datasources.Postgres;

internal static class PostgresBind
{
    static readonly Regex Name = new(@"\$([A-Za-z_][A-Za-z0-9_]*)", RegexOptions.Compiled);

    public static (string Sql, object?[] Values) ToPostgres(string sql, Row parameters)
    {
        var (text, names) = Rewrite(sql);
        return (text, names.Select(name => parameters.TryGetValue(name, out var value) ? Reverse(value).ToObject() : null).ToArray());
    }

    public static (string Sql, Parameter[] Bound) ToPostgres(string sql, IReadOnlyList<Parameter>? parameters)
    {
        var map = new Dictionary<string, Parameter>(StringComparer.Ordinal);
        if (parameters is not null)
            foreach (var parameter in parameters)
                map[parameter.Name.TrimStart('$', '@', ':')] = parameter;
        var (text, names) = Rewrite(sql);
        return (text, names.Select(name => map.TryGetValue(name, out var parameter)
            ? parameter
            : new Parameter(name, Value.Null.Instance)).ToArray());
    }

    static (string Sql, List<string> Names) Rewrite(string sql)
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
        return (text, names);
    }

    public static Value Reverse(Value value) => value switch
    {
        Value.DateTime => new DateTimeConverter().ReverseConvert(value),
        Value.Guid => new UuidConverter().ReverseConvert(value),
        Value.Decimal => new DecimalConverter().ReverseConvert(value),
        Value.Bool => new BoolConverter().ReverseConvert(value),
        Value.Binary => new BinaryConverter().ReverseConvert(value),
        _ => value
    };

    public static string IsolationSql(IsolationLevel isolation) => isolation switch
    {
        IsolationLevel.ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel.ReadCommitted => "READ COMMITTED",
        IsolationLevel.RepeatableRead => "REPEATABLE READ",
        _ => "SERIALIZABLE"
    };
}
