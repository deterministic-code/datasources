namespace Datasources;

public abstract class ReadonlyRepository<T> : IReadonlyRepository<T>
{
    public Type Type => typeof(T);

    public abstract IAsyncEnumerable<T> Read(IStatement? query = null);

    public virtual IAsyncEnumerable<T> Read(IWhere where) =>
        Read(new QueryStatement(where.GetText(), where));

    public virtual async Task<T?> ReadSingle(IStatement? query = null)
    {
        await foreach (var item in Read(query))
            return item;
        return default;
    }

    public virtual Task<T?> ReadSingle(IWhere where) =>
        ReadSingle(new QueryStatement(where.GetText(), where));
}
