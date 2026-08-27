namespace Datasources;

public static class Datasource
{
    private static readonly Dictionary<string, IDatasourceFactory> Factories = new();

    public static void Use(IDatasourceFactory factory) => Factories[factory.Provider] = factory;

    public static IDatasource Create(string provider, object? options = null) =>
        Factories.TryGetValue(provider, out var factory)
            ? factory.Create(options)
            : throw new ProviderNotInstalledException(provider);

    public static void Reset() => Factories.Clear();

    public static IReadOnlyList<string> Registered() => Factories.Keys.OrderBy(k => k).ToArray();
}
