namespace Datasources;

public interface ITransaction : IMiddlewareHost
{
    IConnection Connection { get; }
    IsolationLevel Isolation { get; }
    Task Commit();
    Task Abort();
}
