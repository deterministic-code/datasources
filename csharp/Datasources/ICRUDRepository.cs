namespace Datasources;

public interface ICRUDRepository<T> : IReadonlyRepository<T>
{
    IReadOnlyList<string> Ids { get; }
    Task<T> Create(T item, IWriteEnrichmentStrategy<T>? writeEnrichment = null);
    Task<Response> Update(T item, IWhere? byWhere = null, IWriteEnrichmentStrategy<T>? writeEnrichment = null);
    Task<Response> Delete(T item, IWhere? byWhere = null, IWriteEnrichmentStrategy<T>? writeEnrichment = null);
    Task<T?> ReadSingleById(Value id);
    Task<T?> ReadSingleById(Row ids);
}
