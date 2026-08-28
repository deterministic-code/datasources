using Datasources;

namespace Datasources.Oracle;

public static class OracleConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.SqlProviderTypes(
        new Dictionary<string, string>
        {
        ["VARCHAR2"] = "TEXT",
        ["NVARCHAR2"] = "TEXT",
        ["CHAR"] = "TEXT",
        ["CLOB"] = "TEXT",
        ["NUMBER"] = "NUMERIC",
        ["BINARY_FLOAT"] = "REAL",
        ["BINARY_DOUBLE"] = "REAL",
        ["TIMESTAMP"] = "DATETIME",
        ["DATE"] = "DATETIME",
        ["RAW"] = "BLOB",
        },
        rename: true,
        booleanStorage: "integer");

    public static void Register() => TypeConverter.Use("oracle", Types);
}
