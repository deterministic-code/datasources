using Datasources;

namespace Datasources.SqlServer;

public static class SqlServerConverters
{
    public static ProviderTypes Types { get; } = FieldConverters.Sql(
        "integer",
        new Dictionary<string, string>
        {
            ["NVARCHAR"] = "TEXT",
            ["VARCHAR"] = "TEXT",
            ["NCHAR"] = "TEXT",
            ["CHAR"] = "TEXT",
            ["INT"] = "INTEGER",
            ["BIGINT"] = "INTEGER",
            ["SMALLINT"] = "INTEGER",
            ["TINYINT"] = "INTEGER",
            ["FLOAT"] = "REAL",
            ["DECIMAL"] = "NUMERIC",
            ["BIT"] = "BOOLEAN",
            ["DATETIME2"] = "DATETIME",
            ["DATE"] = "DATETIME",
            ["VARBINARY"] = "BLOB",
            ["UNIQUEIDENTIFIER"] = "UUID",
        });

    public static void Register() => TypeConverter.Use("sqlserver", Types);
}
