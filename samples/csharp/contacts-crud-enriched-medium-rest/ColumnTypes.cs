using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public static class ColumnTypes
{
    public static Func<string, string> Map(string provider) => provider switch
    {
        "sqlite" => Sqlite,
        "mysql" => Mysql,
        "postgres" => Postgres,
        _ => throw new InvalidOperationException($"unknown provider {provider}")
    };

    static string Sqlite(string canonical) => canonical switch
    {
        "int" => "INTEGER",
        "uuid" or "string" or "datetime" => "TEXT",
        "bool" => "INTEGER",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "binary" => "BLOB",
        _ => throw new DatasourceException($"Unknown sqlite type \"{canonical}\"")
    };

    static string Mysql(string canonical) => canonical switch
    {
        "int" => "INT",
        "uuid" => "CHAR(36)",
        "string" or "datetime" => "TEXT",
        "bool" => "TINYINT",
        "decimal" => "DECIMAL(18,4)",
        "float" => "DOUBLE",
        "binary" => "BLOB",
        _ => throw new DatasourceException($"Unknown mysql type \"{canonical}\"")
    };

    static string Postgres(string canonical) => canonical switch
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
