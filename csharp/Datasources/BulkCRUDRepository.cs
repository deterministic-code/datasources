namespace Datasources;

public class BulkCRUDRepository<T> : CRUDRepository<T> where T : new()
{
    public BulkCRUDRepository(IConnection connection) : base(connection) { }

    public BulkCRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) : base(connection, options) { }

    public BulkCRUDRepository(BuiltRepository<T> built) : base(built) { }

    protected BulkCRUDRepository(IWriteEnrichmentStrategy<T>? write = null) : base(write) { }

    public async Task<Response> Create(IReadOnlyList<T> items)
    {
        var result = (Value)Value.Null.Instance;
        foreach (var item in items)
            result = IdValue(await base.Create(item));
        return new Response(result, items.Count);
    }

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        Fold(items, item => base.Update(item, byWhere));

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        Fold(items, item => base.Delete(item, byWhere));

    public Task<Response> BulkCreate(IReadOnlyList<T> items) => Create(items);

    public Task<Response> BulkUpdate(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        Update(items, byWhere);

    public Task<Response> BulkDelete(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        Delete(items, byWhere);

    static async Task<Response> Fold(IReadOnlyList<T> items, Func<T, Task<Response>> write)
    {
        var result = (Value)Value.Null.Instance;
        var rows = 0;
        foreach (var item in items)
        {
            var response = await write(item);
            result = response.Result;
            rows += response.RowsAffected;
        }
        return new Response(result, rows);
    }
}
