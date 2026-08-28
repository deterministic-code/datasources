namespace Datasources;

public interface IConnection
{
    Task Open();
    Task Close();
    Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable);
}
