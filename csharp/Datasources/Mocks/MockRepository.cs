namespace Datasources;

public sealed class MockRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    public List<T> Items { get; } = new();
    public Exception? ReadSingleError { get; set; }
    public Exception? ReadError { get; set; }
    public Exception? CreateError { get; set; }
    public Exception? UpdateError { get; set; }
    public Exception? DeleteError { get; set; }
    public MockRepository(IConnection _) { }

    public override Task<T?> ReadSingle(IStatement? query = null) =>
        ReadSingleError is not null
            ? throw ReadSingleError
            : Task.FromResult(Items.Count == 0 ? default : Items[0]);

    public override async IAsyncEnumerable<T> Read(IStatement? query = null)
    {
        if (ReadError is not null) throw ReadError;
        await Task.Yield();
        foreach (var item in Items) yield return item;
    }

    public Task<Response> Create(T item)
    {
        if (CreateError is not null) throw CreateError;
        Items.Add(item);
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    public Task<Response> Update(T item, IWhere? byWhere = null) =>
        UpdateError is not null ? throw UpdateError : Task.FromResult(new Response(Value.Null.Instance, 1));

    public Task<Response> Delete(T item, IWhere? byWhere = null) =>
        DeleteError is not null ? throw DeleteError : Task.FromResult(new Response(Value.Null.Instance, 1));
}
