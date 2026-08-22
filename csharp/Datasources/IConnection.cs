namespace Datasources;

public interface IConnection : IMiddlewareHost
{
    Task Open();
    Task Close();
    Task<ITransaction> StartTransaction();
}
