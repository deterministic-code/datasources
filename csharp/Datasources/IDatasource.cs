namespace Datasources;

public interface IDatasource : IAsyncDisposable
{
    string Provider { get; }
    RepositoryFactory Repositories { get; }
    Task Close();
    ValueTask IAsyncDisposable.DisposeAsync() => new(Close());
    Task<IConnection> CreateConnection();
    Task<ICommand> CreateCommand(IConnection? connection = null);
    ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null, CRUDRepositoryOptions<T>? options = null) where T : new();
    ICRUDRepository<T> CreateRepository<T>(string typeName, IConnection? connection = null);
    IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null, ReadonlyRepositoryMappingOptions<T>? options = null) where T : new() =>
        CreateRepository<T>(connection, options as CRUDRepositoryOptions<T>);
    IReadonlyRepository<T> CreateReadonlyRepository<T>(string typeName, IConnection? connection = null);
}
