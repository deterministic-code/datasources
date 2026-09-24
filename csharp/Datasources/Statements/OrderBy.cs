namespace Datasources;

public enum SortDirection
{
    Asc,
    Desc,
}

public sealed record OrderBy(string Column, SortDirection Direction)
{
    public static OrderBy Asc(string column) => new(column, SortDirection.Asc);
    public static OrderBy Desc(string column) => new(column, SortDirection.Desc);
    public override string ToString() => $"{Column} {Sql(Direction)}";

    static string Sql(SortDirection direction) => direction switch
    {
        SortDirection.Asc => "ASC",
        SortDirection.Desc => "DESC",
        _ => throw new InvalidOperationException("empty order")
    };
}
