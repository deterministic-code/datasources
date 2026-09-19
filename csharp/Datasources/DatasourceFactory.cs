using System.Reflection;

namespace Datasources;

public sealed class DatasourceFactory : IDatasourceFactory
{
    static readonly Dictionary<string, (Func<IReadOnlyDictionary<string, string?>?, IConnection> Connection, Func<IConnection, object> Repository)> Factories = new();

    public static IDatasource Default => Create();

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
        {
            options = Merge(DotEnv.Load(), Environment.GetEnvironmentVariable);
            RegisterInstalled(options);
        }
        var name = Get(options, "provider") ?? ProviderFromUrl.From(Get(options, "database_url"))
            ?? throw new DatasourceException("Datasource defaults are not specified");
        return Factories.TryGetValue(name, out var pair)
            ? new Datasource(name, () => pair.Connection(options), pair.Repository)
            : throw new ProviderNotInstalledException(name);
    }

    static string? Get(IReadOnlyDictionary<string, string?>? options, string key) =>
        options is not null && options.TryGetValue(key, out var value) && !string.IsNullOrWhiteSpace(value)
            ? value
            : null;

    static string? NonEmpty(string? value) =>
        string.IsNullOrWhiteSpace(value) ? null : value.Trim();

    static IReadOnlyDictionary<string, string?> Merge(
        IReadOnlyDictionary<string, string?> file,
        Func<string, string?> env)
    {
        var options = file.ToDictionary(entry => entry.Key, entry => entry.Value);
        foreach (var key in (string[])["INSTALLED_PROVIDERS", "DATASOURCE_URL", "DATABASE_SCHEMA", "PROVIDER", "provider"])
        {
            var value = NonEmpty(env(key));
            if (value is not null)
                options[key] = value;
        }
        var url = NonEmpty(env("DEFAULT_DATASOURCE_URL")) ?? Get(options, "DATASOURCE_URL");
        var provider = NonEmpty(env("DEFAULT_PROVIDER")) ?? Get(options, "PROVIDER") ?? Get(options, "provider");
        if (url is not null)
        {
            options["DATASOURCE_URL"] = url;
            options["database_url"] = url;
        }
        if (provider is not null)
            options["provider"] = provider;
        var schema = Get(options, "DATABASE_SCHEMA");
        if (schema is not null)
            options["database_schema"] = schema;
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
