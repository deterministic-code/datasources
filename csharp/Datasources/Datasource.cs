namespace Datasources;

public static class Datasource
{
    private static readonly Dictionary<string, IDatasourceFactory> Factories = new();
    private static IReadOnlyDictionary<string, string?>? Specified;

    public static void Defaults(IReadOnlyDictionary<string, string?> options) =>
        Specified = options;

    public static void Use(IDatasourceFactory factory) => Factories[factory.Provider] = factory;

    public static void Use(IDatasourceFactory first, params IDatasourceFactory[] rest)
    {
        Use(first);
        foreach (var factory in rest)
            Use(factory);
    }

    public static IDatasource Create(string? provider = null, object? options = null)
    {
        var name = provider ?? Get(Specified, "provider")
            ?? throw new DatasourceException("Datasource defaults are not specified");
        return Factories.TryGetValue(name, out var factory)
            ? factory.Create(options ?? Specified)
            : throw new ProviderNotInstalledException(name);
    }

    static string? Get(IReadOnlyDictionary<string, string?>? options, string key) =>
        options is not null && options.TryGetValue(key, out var value) ? value : null;

    public static void Reset()
    {
        Factories.Clear();
        Specified = null;
    }

    public static IReadOnlyList<string> Registered() => Factories.Keys.OrderBy(k => k).ToArray();
}
