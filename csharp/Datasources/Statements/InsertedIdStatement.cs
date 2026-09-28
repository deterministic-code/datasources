namespace Datasources;

public sealed record InsertedIdStatement(string Table, string Column) : IStatement
{
    public string GetText() => $"{Table}.{Column}";

    public static string Select(string provider, string column) => provider switch
    {
        "sqlite" => $"SELECT last_insert_rowid() AS {column}",
        "mysql" => $"SELECT LAST_INSERT_ID() AS {column}",
        "postgres" => $"SELECT lastval() AS {column}",
        _ => ""
    };
}
