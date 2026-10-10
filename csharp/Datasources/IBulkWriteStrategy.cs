namespace Datasources;

public interface IBulkWriteStrategy<T> where T : new()
{
    CRUDRepository<T> Repo { get; set; }
    Task<Response> Create(IReadOnlyList<T> items);
    Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere);
    Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere);
}
