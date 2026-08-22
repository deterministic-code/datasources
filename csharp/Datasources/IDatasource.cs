namespace Datasources;

public interface IDatasource
{
    string Provider { get; }
    IConnection CreateConnection();
    ICommand CreateCommand(IConnection? connection = null);
    IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null);
}
