namespace Datasources;

public sealed class RepositoryBuilderOptionsFactory<T> where T : new()
{
    CRUDRepositoryOptions<T>? fallback;
    readonly Dictionary<string, CRUDRepositoryOptions<T>> items = new(StringComparer.Ordinal);

    public RepositoryBuilderOptionsFactory<T> Add(CRUDRepositoryOptions<T> options)
    {
        fallback = options;
        return this;
    }

    public RepositoryBuilderOptionsFactory<T> Add(string dialect, CRUDRepositoryOptions<T> options)
    {
        items[dialect] = options;
        return this;
    }

    public CRUDRepositoryOptions<T> For(string provider) =>
        items.TryGetValue(provider, out var options)
            ? options
            : fallback ?? throw new DatasourceException("repository options required");
}
