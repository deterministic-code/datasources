namespace Datasources;

public sealed class MockTransaction : ITransaction
{
    public int CommitCalls { get; private set; }
    public int AbortCalls { get; private set; }
    public Exception? CommitError { get; set; }
    public Exception? AbortError { get; set; }
    public bool Committed { get; private set; }
    public bool Aborted { get; private set; }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; set; } = IsolationLevel.Serializable;

    public MockTransaction(IConnection connection) => Connection = connection;

    public Task Commit()
    {
        CommitCalls++;
        if (CommitError is not null) throw CommitError;
        Committed = true;
        return Task.CompletedTask;
    }

    public Task Abort()
    {
        AbortCalls++;
        if (AbortError is not null) throw AbortError;
        Aborted = true;
        return Task.CompletedTask;
    }
}
