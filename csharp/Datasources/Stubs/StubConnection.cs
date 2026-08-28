namespace Datasources;

public class StubConnection : IConnection
{
    private readonly string _provider;
    public string Provider => _provider;
    public ICommand CreateCommand() => new StubCommand(_provider, this);
    public StubConnection(string provider) => _provider = provider;
    public Task Open() => throw new ProviderNotImplementedException(_provider);
    public Task Close() => throw new ProviderNotImplementedException(_provider);
    public Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable) =>
        throw new ProviderNotImplementedException(_provider);
}
