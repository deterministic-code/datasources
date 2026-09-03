namespace Datasources;

public class BulkCRUDRepository<T> : DeepCRUDRepository<T>
{
    public BulkCRUDRepository(ICRUDRepository<T> inner) : base(inner) { }

    public Task<Response> Create(IReadOnlyList<T> items) =>
        Fold(items, item => base.Create(item));

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
