namespace Datasources;

public sealed class StubTransaction : MiddlewareHost, ITransaction
{
    private readonly string _provider;
    public StubTransaction(string provider, IConnection connection)
    {
        _provider = provider;
        Connection = connection;
    }

    public IConnection Connection { get; }
    public Task Commit() => Run("commit", () => throw new ProviderNotImplementedException(_provider));
    public Task Abort() => Run("abort", () => throw new ProviderNotImplementedException(_provider));
}
