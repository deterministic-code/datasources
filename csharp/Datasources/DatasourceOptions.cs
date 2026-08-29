namespace Datasources;

public static class DatasourceOptions
{
    public static string? Value(object? options, string key) =>
        options is IReadOnlyDictionary<string, string?> dict && dict.TryGetValue(key, out var value)
            ? value
            : null;
}
