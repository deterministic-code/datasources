using Datasources;

namespace Datasources.Memory;

public sealed class MemoryTransaction : MiddlewareHost, ITransaction
{
    public MemoryTransaction(MemoryConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
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
