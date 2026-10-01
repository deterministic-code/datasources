namespace Datasources;

public interface IReader<out T> : IAsyncDisposable
{
    T? Current { get; }
    Task<bool> Next();
    Task<bool> NextResult() => Task.FromResult(false);
    ValueTask IAsyncDisposable.DisposeAsync() => ValueTask.CompletedTask;

    async IAsyncEnumerable<T> Read()
    {
        while (await Next())
            if (Current is { } current)
                yield return current;
    }
}

public interface IReader : IReader<object>;
    