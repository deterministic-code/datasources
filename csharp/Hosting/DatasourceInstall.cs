using System.Reflection;

namespace Datasources;

public static class DatasourceInstall
{
    static readonly Dictionary<string, (string Assembly, string Type)> Packages = new()
    {
        ["json"] = ("Datasources.Json", "Datasources.Json.JsonDriver"),
        ["memory"] = ("Datasources.Memory", "Datasources.Memory.MemoryDriver"),
        ["sqlite"] = ("Datasources.Sqlite", "Datasources.Sqlite.SqliteDriver"),
        ["xml"] = ("Datasources.Xml", "Datasources.Xml.XmlDriver"),
        ["yaml"] = ("Datasources.Yaml", "Datasources.Yaml.YamlDriver"),
        ["mysql"] = ("Datasources.Mysql", "Datasources.Mysql.MysqlDriver"),
        ["sqlserver"] = ("Datasources.SqlServer", "Datasources.SqlServer.SqlServerDriver"),
        ["postgres"] = ("Datasources.Postgres", "Datasources.Postgres.PostgresDriver"),
        ["oracle"] = ("Datasources.Oracle", "Datasources.Oracle.OracleDriver"),
        ["mongodb"] = ("Datasources.MongoDb", "Datasources.MongoDb.MongoDbDriver"),
        ["dynamodb"] = ("Datasources.DynamoDb", "Datasources.DynamoDb.DynamoDbDriver")
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
