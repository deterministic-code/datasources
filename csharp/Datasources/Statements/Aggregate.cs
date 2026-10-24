namespace Datasources;

public static class Aggregate
{
    public static string Count(string column) => Sql("COUNT", column);
    public static string Sum(string column) => Sql("SUM", column);
    public static string Min(string column) => Sql("MIN", column);
    public static string Max(string column) => Sql("MAX", column);

    static string Sql(string op, string column) => $"{op}({column})";
}
