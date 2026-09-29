namespace Datasources;

public interface IConnection : IAsyncDisposable
{
    string Provider { get; }
    bool IsOpen { get; }
    ICommand CreateCommand();
    Task Open();
    Task Close();
    Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable);
    ValueTask IAsyncDisposable.DisposeAsync() => new(Close());
}
