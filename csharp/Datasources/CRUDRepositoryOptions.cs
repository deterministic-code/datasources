namespace Datasources;

public sealed class CRUDRepositoryOptions<T> : ReadonlyRepositoryOptions<T> where T : new()
{
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
    public IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
    public IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
    public IMapper<IReturnStatement, IReturnStatement>? Return { get; init; }
    public IWriteEnrichmentStrategy<T>? Write { get; init; }
}
