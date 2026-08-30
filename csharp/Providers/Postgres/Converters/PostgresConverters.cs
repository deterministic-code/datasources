using Datasources;

namespace Datasources.Postgres;

public static class PostgresConverters
{
    public static ProviderTypes Types { get; } = FieldConverters.Sql(
        "native",
        new Dictionary<string, string>
        {
            ["VARCHAR"] = "TEXT",
            ["CHAR"] = "TEXT",
            ["INT2"] = "INTEGER",
            ["INT4"] = "INTEGER",
            ["INT8"] = "INTEGER",
            ["FLOAT4"] = "REAL",
            ["FLOAT8"] = "REAL",
            ["DECIMAL"] = "NUMERIC",
            ["TIMESTAMPTZ"] = "DATETIME",
            ["TIMESTAMP"] = "DATETIME",
            ["DATE"] = "DATETIME",
            ["BYTEA"] = "BLOB",
        });

    public static void Register() => TypeConverter.Use("postgres", Types);
}
