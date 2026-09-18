namespace Datasources;

public static class Datasource
{
    private static readonly Dictionary<string, IDatasourceFactory> Factories = new();
    private static IReadOnlyDictionary<string, string?>? Specified;

    public static void Defaults(IReadOnlyDictionary<string, string?> options) =>
        Specified = options;

    public static void Register(IDatasourceFactory factory) => Factories[factory.Provider] = factory;

    public static void Register(IDatasourceFactory first, params IDatasourceFactory[] rest)
    {
        Register(first);
        foreach (var factory in rest)
            Register(factory);
    }

    public static IDatasource Create(string? provider = null, object? options = null)
    {
        var name = provider ?? Get(Specified, "provider")
            ?? ProviderFromUrl.From(Get(Specified, "url") ?? Get(Specified, "DATABASE_URL"))
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
