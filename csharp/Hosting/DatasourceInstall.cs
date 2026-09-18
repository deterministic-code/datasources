using System.Reflection;

namespace Datasources;

public static class DatasourceInstall
{
    static readonly Dictionary<string, (string Assembly, string Type)> Packages = new()
    {
        ["json"] = ("Datasources.Json", "Datasources.Json.JsonDatasource"),
        ["memory"] = ("Datasources.Memory", "Datasources.Memory.MemoryDatasource"),
        ["sqlite"] = ("Datasources.Sqlite", "Datasources.Sqlite.SqliteDatasource"),
        ["text"] = ("Datasources.Text", "Datasources.Text.TextDatasource"),
        ["xml"] = ("Datasources.Xml", "Datasources.Xml.XmlDatasource"),
        ["yaml"] = ("Datasources.Yaml", "Datasources.Yaml.YamlDatasource"),
        ["mysql"] = ("Datasources.Mysql", "Datasources.Mysql.MysqlDatasource"),
        ["sqlserver"] = ("Datasources.SqlServer", "Datasources.SqlServer.SqlServerDatasource"),
        ["postgres"] = ("Datasources.Postgres", "Datasources.Postgres.PostgresDatasource"),
        ["oracle"] = ("Datasources.Oracle", "Datasources.Oracle.OracleDatasource"),
        ["mongodb"] = ("Datasources.MongoDb", "Datasources.MongoDb.MongoDbDatasource"),
        ["dynamodb"] = ("Datasources.DynamoDb", "Datasources.DynamoDb.DynamoDbDatasource")
    };

    public static void From(IReadOnlyDictionary<string, string?> options)
    {
        foreach (var name in ProviderNames.From(options))
            Datasource.Register(Resolve(name));
        Datasource.Defaults(options);
    }

    static IDatasourceFactory Resolve(string name) =>
        Packages.TryGetValue(name, out var package)
            ? Load(package, name)
            : throw new ProviderNotInstalledException(name);

    static IDatasourceFactory Load((string Assembly, string Type) package, string name)
    {
        try
        {
            var type = Type.GetType($"{package.Type}, {package.Assembly}", throwOnError: false)
                ?? Assembly.Load(package.Assembly).GetType(package.Type);
            return type?.GetField("Instance", BindingFlags.Public | BindingFlags.Static)?.GetValue(null)
                is IDatasourceFactory factory
                ? factory
                : throw new ProviderNotInstalledException(name);
        }
        catch (ProviderNotInstalledException)
        {
            throw;
        }
        catch (Exception)
        {
            throw new ProviderNotInstalledException(name);
        }
    }
}
