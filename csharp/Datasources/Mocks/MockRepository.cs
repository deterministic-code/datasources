namespace Datasources;

public sealed class MockRepository<T> : IRepository<T>
{
    public List<T> Items { get; } = new();
    public Exception? GetSingleError { get; set; }
    public Exception? GetError { get; set; }
    public Exception? AddError { get; set; }
    public Exception? UpdateError { get; set; }
    public Exception? DeleteError { get; set; }
    public MockRepository(IConnection _) { }

    public Task<T?> GetSingle(IWhere where) => GetSingle((IQuery?)null);

    public IAsyncEnumerable<T> Get(IWhere where) => Get((IQuery?)null);

    public Task<T?> GetSingle(IQuery? query = null) =>
        GetSingleError is not null
            ? throw GetSingleError
            : Task.FromResult(Items.Count == 0 ? default : Items[0]);

    public async IAsyncEnumerable<T> Get(IQuery? query = null)
    {
        if (GetError is not null) throw GetError;
        await Task.Yield();
        foreach (var item in Items) yield return item;
    }

    public Task<Response> Add(T obj)
    {
        if (AddError is not null) throw AddError;
        Items.Add(obj);
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    public Task<Response> Update(T obj, IWhere? byWhere = null) =>
        UpdateError is not null ? throw UpdateError : Task.FromResult(new Response(Value.Null.Instance, 1));

    public Task<Response> Delete(T obj, IWhere? byWhere = null) =>
        DeleteError is not null ? throw DeleteError : Task.FromResult(new Response(Value.Null.Instance, 1));
}
