using Datasources;

namespace Datasources.Memory;

public sealed class MemoryTransaction : MiddlewareHost, ITransaction
{
    public MemoryTransaction(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task Commit() => Run("commit", () =>
    {
        Typed.FinishCommit();
        return Task.CompletedTask;
    });

    public Task Abort() => Run("abort", () =>
    {
        Typed.FinishAbort();
        return Task.CompletedTask;
    });
}
