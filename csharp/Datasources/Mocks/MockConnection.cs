namespace Datasources;

public sealed class MockConnection : MiddlewareHost, IConnection
{
    public int OpenCalls { get; private set; }
    public int CloseCalls { get; private set; }
    public int StartTransactionCalls { get; private set; }
    public Exception? OpenError { get; set; }
    public Exception? CloseError { get; set; }
    public Exception? StartTransactionError { get; set; }
    public bool Opened { get; private set; }
    public ITransaction Transaction { get; }

    public MockConnection(ITransaction? transaction = null)
    {
        Transaction = transaction ?? new MockTransaction(this);
    }

    public Task Open() => Run("open", () =>
    {
        OpenCalls++;
        if (OpenError is not null) throw OpenError;
        Opened = true;
        return Task.CompletedTask;
    });

    public Task Close() => Run("close", () =>
    {
        CloseCalls++;
        if (CloseError is not null) throw CloseError;
        Opened = false;
        return Task.CompletedTask;
    });

    public Task<ITransaction> StartTransaction() => Run("startTransaction", () =>
    {
        StartTransactionCalls++;
        if (StartTransactionError is not null) throw StartTransactionError;
        return Task.FromResult(Transaction);
    });
}
