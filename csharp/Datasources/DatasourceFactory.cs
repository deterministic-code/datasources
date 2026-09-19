using System.Reflection;

namespace Datasources;

public sealed class DatasourceFactory : IDatasourceFactory
{
    static readonly Dictionary<string, (Func<IReadOnlyDictionary<string, string?>?, IConnection> Connection, Func<IConnection, object> Repository)> Factories = new();

    public static void Defaults(IReadOnlyDictionary<string, string?> options) => RegisterInstalled(options);

    public static void Register(
        string provider,
        Func<IReadOnlyDictionary<string, string?>?, IConnection> connection,
        Func<IConnection, object> repository) =>
        Factories[provider] = (connection, repository);

    public static IDatasource Create(IReadOnlyDictionary<string, string?>? options = null) =>
        Resolve(options);

    IDatasource IDatasourceFactory.Create(IReadOnlyDictionary<string, string?>? options) =>
        Resolve(options);

    static IDatasource Resolve(IReadOnlyDictionary<string, string?>? options)
    {
        if (options is null)
            RegisterInstalled(InstallFromEnvironment());
        var listed = options ?? ConnectionFromEnvironment();
        var name = Get(listed, "provider") ?? ProviderFromUrl.From(Get(listed, "database_url"))
            ?? throw new DatasourceException("Datasource defaults are not specified");
        return Factories.TryGetValue(name, out var pair)
            ? new Datasource(name, () => pair.Connection(listed), pair.Repository)
            : throw new ProviderNotInstalledException(name);
    }

    static string? Get(IReadOnlyDictionary<string, string?>? options, string key) =>
        options is not null && options.TryGetValue(key, out var value) && !string.IsNullOrWhiteSpace(value)
            ? value
            : null;

    static IReadOnlyDictionary<string, string?> ConnectionFromEnvironment()
    {
        var options = new Dictionary<string, string?>();
        var url = Environment.GetEnvironmentVariable("DATABASE_URL");
        if (!string.IsNullOrWhiteSpace(url))
            options["database_url"] = url;
        var schema = Environment.GetEnvironmentVariable("DATABASE_SCHEMA");
        if (!string.IsNullOrWhiteSpace(schema))
            options["database_schema"] = schema;
        return options;
    }

    static IReadOnlyDictionary<string, string?> InstallFromEnvironment()
    {
        var options = new Dictionary<string, string?>();
        foreach (var key in (string[])["INSTALLED_PROVIDERS", "DATABASE_URL"])
        {
            var value = Environment.GetEnvironmentVariable(key);
            if (!string.IsNullOrWhiteSpace(value))
                options[key] = value;
        }
        return options;
    }

    static void RegisterInstalled(IReadOnlyDictionary<string, string?>? options)
    {
        if (options is null) return;
        var method = Type.GetType("Datasources.DatasourceInstall, Datasources.Hosting")
            ?.GetMethod("RegisterListed", BindingFlags.Public | BindingFlags.Static);
        if (method is null) return;
        try
        {
            method.Invoke(null, [options]);
        }
        catch (TargetInvocationException ex)
        {
            throw ex.InnerException ?? ex;
        }
    }

    public static void Reset() => Factories.Clear();

    public static IReadOnlyList<string> Registered() => Factories.Keys.OrderBy(k => k).ToArray();
}
