namespace Datasources;

public sealed class MockRepository<T> : IRepository<T>
{
    public List<T> Items { get; } = new();
    public Exception? GetSingleError { get; set; }
    public Exception? GetError { get; set; }
    public Exception? AddError { get; set; }
    public Exception? UpdateError { get; set; }
    public Exception? DeleteError { get; set; }
    public IConnection Connection { get; }
    public MockRepository(IConnection connection) => Connection = connection;

    public Task<T?> GetSingle(IQuery? query = null)
    {
        if (GetSingleError is not null) throw GetSingleError;
        return Task.FromResult(Items.Count == 0 ? default : Items[0]);
    }

    public async IAsyncEnumerable<T> Get(IQuery? query = null)
    {
        if (GetError is not null) throw GetError;
        await Task.Yield();
        foreach (var item in Items)
        {
            yield return item;
        }
    }

    public Task<Response> Add(T obj)
    {
        if (AddError is not null) throw AddError;
        Items.Add(obj);
        return Task.FromResult(new Response(obj, 1));
    }

    public Task<Response> Update(T obj, IWhere? byWhere = null)
    {
        if (UpdateError is not null) throw UpdateError;
        return Task.FromResult(new Response(obj, 1));
    }

    public Task<Response> Delete(T obj, IWhere? byWhere = null)
    {
        if (DeleteError is not null) throw DeleteError;
        return Task.FromResult(new Response(null, 1));
    }
}
