namespace Datasources.Mysql;

public static class MysqlColumnType
{
    public static string Map(string canonical) => canonical switch
    {
        "int" => "INT",
        "uuid" => "CHAR(36)",
        "string" => "TEXT",
        "bool" => "TINYINT",
        "decimal" => "DECIMAL(18,4)",
        "float" => "DOUBLE",
        "datetime" => "TEXT",
        "binary" => "BLOB",
        _ => throw new DatasourceException($"Unknown mysql type \"{canonical}\"")
    };
}
