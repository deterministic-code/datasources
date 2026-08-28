namespace Datasources;

public interface IDatasource
{
    string Provider { get; }
    Task<IConnection> CreateConnection();
    ICommand CreateCommand(IConnection? connection = null);
    IRepository<T> CreateRepository<T>(IConnection? connection = null);
    IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null) =>
        CreateRepository<T>(connection);
}
