namespace Datasources;

public interface IConnection
{
    string Provider { get; }
    ICommand CreateCommand();
    Task Open();
    Task Close();
    Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable);
}
