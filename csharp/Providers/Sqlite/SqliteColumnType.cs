namespace Datasources.Sqlite;

public static class SqliteColumnType
{
    public static string Map(string canonical) => canonical switch
    {
        "int" => "INTEGER",
        "uuid" => "TEXT",
        "string" => "TEXT",
        "bool" => "INTEGER",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "datetime" => "TEXT",
        "binary" => "BLOB",
        _ => throw new DatasourceException($"Unknown sqlite type \"{canonical}\"")
    };
}
