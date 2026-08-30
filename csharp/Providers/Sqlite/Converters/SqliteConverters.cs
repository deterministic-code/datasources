using Datasources;

namespace Datasources.Sqlite;

public static class SqliteConverters
{
    public static ProviderTypes Types { get; } = FieldConverters.Sql(
        "integer",
        new Dictionary<string, string>
        {
            ["CHAR"] = "TEXT",
            ["VARCHAR"] = "TEXT",
            ["TINYINT"] = "INTEGER",
            ["SMALLINT"] = "INTEGER",
            ["INT"] = "INTEGER",
            ["BIGINT"] = "INTEGER",
            ["FLOAT"] = "REAL",
            ["DOUBLE"] = "REAL",
            ["DATE"] = "DATETIME",
        });

    public static void Register() => TypeConverter.Use("sqlite", Types);
}
