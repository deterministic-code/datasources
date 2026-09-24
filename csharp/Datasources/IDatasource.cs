namespace Datasources;

public interface IDatasource
{
    string Provider { get; }
    Task<IConnection> CreateConnection();
    ICommand CreateCommand(IConnection? connection = null);
    ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null, CRUDRepositoryOptions<T>? options = null) where T : new();
    IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null, ReadonlyRepositoryOptions<T>? options = null) where T : new() =>
        CreateRepository<T>(connection, options as CRUDRepositoryOptions<T>);
}
