namespace Datasources;

public sealed class SingleCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
{
    public CRUDRepository<T> Repo { get; set; } = null!;

    public async Task<Response> Create(IReadOnlyList<T> items)
    {
        foreach (var item in items)
            await Repo.Create(item);
        return new Response(Value.Null.Instance, items.Count);
    }

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere) =>
        Fold(items, item => Repo.Update(item, byWhere));

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere) =>
        Fold(items, item => Repo.Delete(item, byWhere));

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
