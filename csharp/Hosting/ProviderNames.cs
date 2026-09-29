namespace Datasources;

public static class ProviderNames
{
    public static IReadOnlyList<string> From(IReadOnlyDictionary<string, string?> options)
    {
        var names = new List<string>();
        foreach (var name in Split(Get(options, "INSTALLED_PROVIDERS")))
            if (!names.Contains(name))
                names.Add(name);
        var url = Get(options, "DATASOURCE_URL");
        if (url is null)
            return names;
        var provider = ProviderFromUrl.From(url) ?? throw new DatasourceException("DATASOURCE_URL has no provider");
        return names.Contains(provider) ? names : [..names, provider];
    }

    static string? Get(IReadOnlyDictionary<string, string?> options, string key) =>
        options.TryGetValue(key, out var value) && !string.IsNullOrWhiteSpace(value)
            ? value.Trim()
            : null;

    static string[] Split(string? value) =>
        (value ?? "").Split(',', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
}
