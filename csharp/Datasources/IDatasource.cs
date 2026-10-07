namespace Datasources;

public interface IDatasource
{
    string Provider { get; }
    RepositoryFactory Repositories { get; }
    Task<IConnection> CreateConnection();
    Task<ICommand> CreateCommand(IConnection? connection = null);
    ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null, CRUDRepositoryOptions<T>? options = null) where T : new();
    object CreateRepository(string typeName, IConnection? connection = null);
    IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null, ReadonlyRepositoryMappingOptions<T>? options = null) where T : new() =>
        CreateRepository<T>(connection, options as CRUDRepositoryOptions<T>);
    object CreateReadonlyRepository(string typeName, IConnection? connection = null);
}
