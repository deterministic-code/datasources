using System.Reflection;

namespace Datasources;

public static class Datasource
{
    static readonly string[] EnvKeys = ["INSTALLED_PROVIDERS", "DATABASE_URL", "DATABASE_SCHEMA"];
    private static readonly Dictionary<string, IDatasourceFactory> Factories = new();
    private static IReadOnlyDictionary<string, string?>? Specified;

    public static void Defaults(IReadOnlyDictionary<string, string?> options)
    {
        RegisterInstalled(options);
        Specified = options;
    }

    public static void Register(IDatasourceFactory factory) => Factories[factory.Provider] = factory;

    public static void Register(IDatasourceFactory first, params IDatasourceFactory[] rest)
    {
        Register(first);
        foreach (var factory in rest)
            Register(factory);
    }

    public static IDatasource Create(string? provider = null, object? options = null)
    {
        var listed = options as IReadOnlyDictionary<string, string?> ?? Specified ?? FromEnvironment();
        if (Specified is null && options is not IReadOnlyDictionary<string, string?> && listed.Count > 0)
            Defaults(listed);
        else
            RegisterInstalled(listed);
        var name = provider ?? ProviderFromUrl.From(Get(listed, "DATABASE_URL"))
            ?? throw new DatasourceException("Datasource defaults are not specified");
        return Factories.TryGetValue(name, out var factory)
            ? factory.Create(options is IReadOnlyDictionary<string, string?> or null ? ForFactory(listed) : options)
            : throw new ProviderNotInstalledException(name);
    }

    static string? Get(IReadOnlyDictionary<string, string?>? options, string key) =>
        options is not null && options.TryGetValue(key, out var value) ? value : null;

    static Dictionary<string, string?> ForFactory(IReadOnlyDictionary<string, string?> options)
    {
        var copy = new Dictionary<string, string?>(options);
        copy.TryAdd("url", Get(options, "DATABASE_URL"));
        copy.TryAdd("schema", Get(options, "DATABASE_SCHEMA"));
        return copy;
    }

    static IReadOnlyDictionary<string, string?> FromEnvironment()
    {
        var options = new Dictionary<string, string?>();
        foreach (var key in EnvKeys)
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

    public static void Reset()
    {
        Factories.Clear();
        Specified = null;
    }

    public static IReadOnlyList<string> Registered() => Factories.Keys.OrderBy(k => k).ToArray();
}
