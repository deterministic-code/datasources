namespace Datasources.Postgres;

public static class PostgresColumnType
{
    public static string Map(string canonical) => canonical switch
    {
        "int" => "INTEGER",
        "uuid" => "UUID",
        "string" => "TEXT",
        "bool" => "BOOLEAN",
        "decimal" => "NUMERIC",
        "float" => "DOUBLE PRECISION",
        "datetime" => "TIMESTAMPTZ",
        "binary" => "BYTEA",
        _ => throw new DatasourceException($"Unknown postgres type \"{canonical}\"")
    };
}
