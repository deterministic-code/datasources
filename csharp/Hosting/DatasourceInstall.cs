using System.Reflection;

namespace Datasources;

public static class DatasourceInstall
{
    static readonly Dictionary<string, (string Assembly, string Type)> Packages = new()
    {
        ["json"] = ("Datasources.Json", "Datasources.Json.JsonConnection"),
        ["memory"] = ("Datasources.Memory", "Datasources.Memory.MemoryConnection"),
        ["sqlite"] = ("Datasources.Sqlite", "Datasources.Sqlite.SqliteConnection"),
        ["text"] = ("Datasources.Text", "Datasources.Text.TextConnection"),
        ["xml"] = ("Datasources.Xml", "Datasources.Xml.XmlConnection"),
        ["yaml"] = ("Datasources.Yaml", "Datasources.Yaml.YamlConnection"),
        ["mysql"] = ("Datasources.Mysql", "Datasources.Mysql.MysqlConnection"),
        ["sqlserver"] = ("Datasources.SqlServer", "Datasources.SqlServer.SqlServerConnection"),
        ["postgres"] = ("Datasources.Postgres", "Datasources.Postgres.PostgresConnection"),
        ["oracle"] = ("Datasources.Oracle", "Datasources.Oracle.OracleConnection"),
        ["mongodb"] = ("Datasources.MongoDb", "Datasources.MongoDb.MongoDbConnection"),
        ["dynamodb"] = ("Datasources.DynamoDb", "Datasources.DynamoDb.DynamoDbConnection")
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
            var ctor = type?.GetConstructor([typeof(IReadOnlyDictionary<string, string?>)]);
            var repo = type?.GetMethod("Repository", BindingFlags.Public | BindingFlags.Static, [typeof(IConnection)]);
            if (ctor is null || repo is null)
                throw new ProviderNotInstalledException(name);
            DatasourceFactory.Register(
                name,
                opts => ctor.Invoke([opts]) is IConnection connection
                    ? connection
                    : throw new ProviderNotInstalledException(name),
                conn => repo.Invoke(null, [conn])!);
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
