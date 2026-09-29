namespace Datasources;

public interface IReader<out T> : IAsyncDisposable
{
    T? Current { get; }
    Task<bool> Next();
    Task<bool> NextResult() => Task.FromResult(false);
    ValueTask IAsyncDisposable.DisposeAsync() => ValueTask.CompletedTask;
}

public interface IReader : IReader<object>;
    