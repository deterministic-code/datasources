using Datasources;

namespace Datasources.Mysql;

public static class MysqlConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.SqlProviderTypes(
        new Dictionary<string, string>
        {
        ["VARCHAR"] = "TEXT",
        ["CHAR"] = "TEXT",
        ["TINYTEXT"] = "TEXT",
        ["MEDIUMTEXT"] = "TEXT",
        ["LONGTEXT"] = "TEXT",
        ["TINYINT"] = "INTEGER",
        ["SMALLINT"] = "INTEGER",
        ["MEDIUMINT"] = "INTEGER",
        ["INT"] = "INTEGER",
        ["BIGINT"] = "INTEGER",
        ["FLOAT"] = "REAL",
        ["DOUBLE"] = "REAL",
        ["DECIMAL"] = "NUMERIC",
        ["TINYINT(1)"] = "BOOLEAN",
        ["TIMESTAMP"] = "DATETIME",
        ["DATE"] = "DATETIME",
        ["BINARY"] = "BLOB",
        ["CHAR(36)"] = "UUID",
        },
        rename: true,
        booleanStorage: "integer");

    public static void Register() => TypeConverter.Use("mysql", Types);
}
