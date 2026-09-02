namespace Datasources;

public abstract class ReadonlyRepository<T> : IReadonlyRepository<T>
{
    public abstract IAsyncEnumerable<T> Get(IStatement? query = null);

    public virtual async Task<T?> GetSingle(IStatement? query = null)
    {
        await foreach (var item in Get(query))
            return item;
        return default;
    }
}
