namespace Datasources;

public sealed record CRUDRepositoryOptions<T> : ReadonlyRepositoryOptions<T> where T : new()
{
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public IWriteEnrichmentStrategy<T>? Write { get; init; }
}
