namespace Datasources;

public sealed record ProviderCatalog(string Name, string Location, string Module)
{
    public static IReadOnlyDictionary<string, ProviderCatalog> Default { get; } =
        new ProviderCatalog[]
        {
            new("json", "Datasources.Json", "Datasources.Json.JsonDriver"),
            new("memory", "Datasources.Memory", "Datasources.Memory.MemoryDriver"),
            new("sqlite", "Datasources.Sqlite", "Datasources.Sqlite.SqliteDriver"),
            new("xml", "Datasources.Xml", "Datasources.Xml.XmlDriver"),
            new("yaml", "Datasources.Yaml", "Datasources.Yaml.YamlDriver"),
            new("mysql", "Datasources.Mysql", "Datasources.Mysql.MysqlDriver"),
            new("sqlserver", "Datasources.SqlServer", "Datasources.SqlServer.SqlServerDriver"),
            new("postgres", "Datasources.Postgres", "Datasources.Postgres.PostgresDriver"),
            new("oracle", "Datasources.Oracle", "Datasources.Oracle.OracleDriver"),
            new("mongodb", "Datasources.MongoDb", "Datasources.MongoDb.MongoDbDriver"),
            new("dynamodb", "Datasources.DynamoDb", "Datasources.DynamoDb.DynamoDbDriver"),
        }.ToDictionary(catalog => catalog.Name);
}
