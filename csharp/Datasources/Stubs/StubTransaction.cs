namespace Datasources;

public sealed class StubTransaction : ITransaction
{
    private readonly string _provider;
    public StubTransaction(string provider, IConnection connection)
    {
        _provider = provider;
        Connection = connection;
    }

    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; } = IsolationLevel.Serializable;
    public Task Commit() => throw new ProviderNotImplementedException(_provider);
    public Task Abort() => throw new ProviderNotImplementedException(_provider);
}
