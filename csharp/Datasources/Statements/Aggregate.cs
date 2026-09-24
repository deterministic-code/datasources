namespace Datasources;

public static class Aggregate
{
    public static string Count(string column, string? alias = null) => Sql("COUNT", column, alias);
    public static string Sum(string column, string? alias = null) => Sql("SUM", column, alias);
    public static string Min(string column, string? alias = null) => Sql("MIN", column, alias);
    public static string Max(string column, string? alias = null) => Sql("MAX", column, alias);

    static string Sql(string op, string column, string? alias) =>
        alias is null ? $"{op}({column})" : $"{op}({column}) AS {alias}";
}
