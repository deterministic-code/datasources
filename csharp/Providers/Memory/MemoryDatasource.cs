using Datasources;

namespace Datasources.Memory;

public sealed class MemoryDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MemoryDatasource Instance = new();
    private readonly MemoryConnectionOptions _options;
    public string Provider => "memory";
    public MemoryDatasource(MemoryConnectionOptions? options = null)
    {
        options ??= new MemoryConnectionOptions();
        options.Store ??= new MemoryStore();
        _options = options;
    }

    MemoryConnection? _connection;

    public IDatasource Create(object? options = null) =>
        new MemoryDatasource(options as MemoryConnectionOptions ?? new MemoryConnectionOptions());

    MemoryConnection NewConnection() => new(_options);
    MemoryConnection DefaultConnection() => _connection ??= NewConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(DefaultConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new MemoryCommand(connection as MemoryConnection ?? DefaultConnection());

    public ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (ICRUDRepository<T>)(object)new MemoryRepository(connection as MemoryConnection ?? DefaultConnection());
}
