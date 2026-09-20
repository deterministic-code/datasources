namespace Datasources;

public abstract class ReadonlyRepository<T> : IReadonlyRepository<T>
{
    public Type Type => typeof(T);

    public abstract IAsyncEnumerable<T> Read(IStatement? query = null);

    public virtual async Task<T?> ReadSingle(IStatement? query = null)
    {
        await foreach (var item in Read(query))
            return item;
        return default;
    }
}
