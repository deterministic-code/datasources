namespace Datasources;

public class StubConnection : MiddlewareHost, IConnection
{
    private readonly string _provider;
    public StubConnection(string provider) => _provider = provider;
    public Task Open() => Run("open", () => throw new ProviderNotImplementedException(_provider));
    public Task Close() => Run("close", () => throw new ProviderNotImplementedException(_provider));
    public Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable) =>
        Run<ITransaction>("startTransaction", () => throw new ProviderNotImplementedException(_provider));
}
