namespace Datasources;

public interface ITransaction
{
    IConnection Connection { get; }
    IsolationLevel Isolation { get; }
    Task Commit();
    Task Abort();
}
