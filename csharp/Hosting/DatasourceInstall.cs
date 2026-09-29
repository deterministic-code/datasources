using System.Reflection;

namespace Datasources;

public static class DatasourceInstall
{
    static readonly Dictionary<string, (string Assembly, string Type)> Packages = new()
    {
        ["json"] = ("Datasources.Json", "Datasources.Json.JsonProvider"),
        ["memory"] = ("Datasources.Memory", "Datasources.Memory.MemoryProvider"),
        ["sqlite"] = ("Datasources.Sqlite", "Datasources.Sqlite.SqliteDriver"),
        ["xml"] = ("Datasources.Xml", "Datasources.Xml.XmlProvider"),
        ["yaml"] = ("Datasources.Yaml", "Datasources.Yaml.YamlProvider"),
        ["mysql"] = ("Datasources.Mysql", "Datasources.Mysql.MysqlDriver"),
        ["sqlserver"] = ("Datasources.SqlServer", "Datasources.SqlServer.SqlServerProvider"),
        ["postgres"] = ("Datasources.Postgres", "Datasources.Postgres.PostgresDriver"),
        ["oracle"] = ("Datasources.Oracle", "Datasources.Oracle.OracleProvider"),
        ["mongodb"] = ("Datasources.MongoDb", "Datasources.MongoDb.MongoDbProvider"),
        ["dynamodb"] = ("Datasources.DynamoDb", "Datasources.DynamoDb.DynamoDbProvider")
    };

    public static void From(IReadOnlyDictionary<string, string?> options) =>
        RegisterListed(options);

    public static void RegisterListed(IReadOnlyDictionary<string, string?> options)
    {
        foreach (var name in ProviderNames.From(options))
            Register(name);
    }

    static void Register(string name)
    {
        if (!Packages.TryGetValue(name, out var package))
            throw new ProviderNotInstalledException(name);
        try
        {
            var type = Type.GetType($"{package.Type}, {package.Assembly}", throwOnError: false)
                ?? Assembly.Load(package.Assembly).GetType(package.Type);
            if (Activator.CreateInstance(type!) is not IDriver driver)
                throw new ProviderNotInstalledException(name);
            DatasourceFactory.Register(name, driver);
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
