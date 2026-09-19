namespace Datasources;

public static class DatasourceOptions
{
    public static string? Value(IReadOnlyDictionary<string, string?>? options, string key) =>
        options is not null && options.TryGetValue(key, out var value) ? value : null;
}
