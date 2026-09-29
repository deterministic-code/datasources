using Datasources;

namespace Datasources.Memory;

internal sealed class MemoryTransaction : ITransaction
{
    public MemoryTransaction(MemoryConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task Commit()
    {
        Typed.FinishCommit();
        return Task.CompletedTask;
    }

    public Task Abort()
    {
        Typed.FinishAbort();
        return Task.CompletedTask;
    }
}
