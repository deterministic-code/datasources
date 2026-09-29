namespace Datasources;

public sealed class DatasourceFactory : IDatasourceFactory
{
    static readonly Dictionary<string, IDriver> Factories = new();

    public static void Defaults(IReadOnlyDictionary<string, string?> options) => RegisterInstalled(options);

    public static void Register(string provider, IDriver driver) =>
        Factories[provider] = driver;

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
        var name = Get(options, "PROVIDER")
            ?? throw new DatasourceException("Datasource defaults are not specified");
        return Factories.TryGetValue(name, out var driver)
            ? new Datasource(name, driver, options)
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
        foreach (var key in (string[])["INSTALLED_PROVIDERS", "DATASOURCE_URL", "PROVIDER"])
        {
            var value = NonEmpty(env(key));
            if (value is not null)
                options[key] = value;
        }
        return options;
    }

    static void RegisterInstalled(IReadOnlyDictionary<string, string?>? options)
    {
        if (options is null) return;
        foreach (var name in Installed(options))
            Register(name, DriverLoader.Load(Catalog(name)));
    }

    static ProviderCatalog Catalog(string name) =>
        ProviderCatalog.Default.TryGetValue(name, out var catalog)
            ? catalog
            : throw new ProviderNotInstalledException(name);

    static IEnumerable<string> Installed(IReadOnlyDictionary<string, string?> options)
    {
        var names = new HashSet<string>();
        foreach (var name in Split(Get(options, "INSTALLED_PROVIDERS")))
            if (names.Add(name))
                yield return name;
        var url = Get(options, "DATASOURCE_URL")?.Trim();
        if (string.IsNullOrEmpty(url)) yield break;
        var provider = DatasourceUrlParser.Parse(url)?.Provider
            ?? throw new DatasourceException("DATASOURCE_URL has no provider");
        if (names.Add(provider))
            yield return provider;
    }

    static string[] Split(string? value) =>
        (value ?? "").Split(',', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);

    public static void Reset() => Factories.Clear();

    public static IReadOnlyList<string> Registered() => Factories.Keys.OrderBy(k => k).ToArray();
}
