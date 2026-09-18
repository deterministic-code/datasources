namespace Datasources;

public sealed class MockConnection : IConnection
{
    public int OpenCalls { get; private set; }
    public int CloseCalls { get; private set; }
    public int StartTransactionCalls { get; private set; }
    public Exception? OpenError { get; set; }
    public Exception? CloseError { get; set; }
    public Exception? StartTransactionError { get; set; }
    public bool IsOpen { get; private set; }
    public ITransaction Transaction { get; }
    public string Provider { get; set; } = "mock";
    public ICommand? Command { get; set; }
    public ICommand CreateCommand() => Command ?? new MockCommand(this);

    public MockConnection(ITransaction? transaction = null) =>
        Transaction = transaction ?? new MockTransaction(this);

    public Task Open()
    {
        OpenCalls++;
        if (OpenError is not null) throw OpenError;
        IsOpen = true;
        return Task.CompletedTask;
    }

    public Task Close()
    {
        CloseCalls++;
        if (CloseError is not null) throw CloseError;
        IsOpen = false;
        return Task.CompletedTask;
    }

    public Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        StartTransactionCalls++;
        if (StartTransactionError is not null) throw StartTransactionError;
        if (Transaction is MockTransaction mock) mock.Isolation = isolation;
        return Task.FromResult(Transaction);
    }
}
