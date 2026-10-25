namespace Datasources;

public sealed record CRUDRepositoryOptions<T> : ReadonlyRepositoryMappingOptions<T> where T : new()
{
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public IReadOnlyList<string>? CreateProperties { get; init; }
    public IReadOnlyList<string>? UpdateProperties { get; init; }
    public IWriteEnrichmentStrategy<T>? WriteEnrichment { get; init; }
}
