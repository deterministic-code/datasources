namespace Datasources;

public static class ProviderNames
{
    public static IReadOnlyList<string> From(IReadOnlyDictionary<string, string?> options)
    {
        var names = new List<string>();
        foreach (var name in Split(Get(options, "providers") ?? Get(options, "DATASOURCE_PROVIDERS")))
            if (!names.Contains(name))
                names.Add(name);
        var provider = Get(options, "provider") ?? Get(options, "DATASOURCE_PROVIDER");
        return provider is { Length: > 0 } && !names.Contains(provider)
            ? [..names, provider]
            : names;
    }

    static string? Get(IReadOnlyDictionary<string, string?> options, string key) =>
        options.TryGetValue(key, out var value) && !string.IsNullOrWhiteSpace(value)
            ? value.Trim()
            : null;

    static string[] Split(string? value) =>
        (value ?? "").Split(',', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
}
