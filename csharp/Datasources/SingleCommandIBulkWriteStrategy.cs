namespace Datasources;

public sealed class SingleCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
{
    public async Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo)
    {
        var result = (Value)Value.Null.Instance;
        foreach (var item in items)
            result = await repo.CreateOne(item);
        return new Response(result, items.Count);
    }

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo) =>
        Fold(items, item => repo.UpdateOne(item, byWhere));

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo) =>
        Fold(items, item => repo.DeleteOne(item, byWhere));

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
