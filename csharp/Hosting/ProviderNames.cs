namespace Datasources;

public static class ProviderNames
{
    public static IReadOnlyList<string> From(IReadOnlyDictionary<string, string?> options)
    {
        var names = new List<string>();
        foreach (var name in Split(Get(options, "INSTALLED_PROVIDERS")))
            if (!names.Contains(name))
                names.Add(name);
        var provider = ProviderFromUrl.From(Get(options, "DATABASE_URL"));
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
