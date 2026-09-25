namespace Datasources;

public sealed class StreamingReader : IReader<Row>, IAsyncDisposable
{
    readonly Func<Task<Row?>> read;
    readonly Func<Task<bool>> nextResult;
    readonly Func<Task> dispose;
    bool disposed;

    public StreamingReader(Func<Task<Row?>> read, Func<Task<bool>> nextResult, Func<Task> dispose) =>
        (this.read, this.nextResult, this.dispose) = (read, nextResult, dispose);

    public Row? Current { get; private set; }

    public async Task<bool> Next()
    {
        Current = await read();
        return Current is not null;
    }

    public async Task<bool> NextResult()
    {
        Current = null;
        return await nextResult();
    }

    public async ValueTask DisposeAsync()
    {
        if (disposed) return;
        disposed = true;
        await dispose();
    }
}
