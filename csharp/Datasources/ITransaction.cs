namespace Datasources;

public interface ITransaction : IMiddlewareHost
{
    IConnection Connection { get; }
    Task Commit();
    Task Abort();
}
