namespace Datasources;

public sealed record CRUDRepositoryOptions<T> : ReadonlyRepositoryMappingOptions<T> where T : new()
{
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public IWriteEnrichmentStrategy<T>? Write { get; init; }
}
