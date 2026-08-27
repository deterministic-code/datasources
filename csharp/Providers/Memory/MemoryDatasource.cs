using Datasources;

namespace Datasources.Memory;

public sealed class MemoryDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MemoryDatasource Factory = new();
    private readonly MemoryConnectionOptions _options;
    public string Provider => "memory";
    public MemoryDatasource(MemoryConnectionOptions? options = null)
    {
        options ??= new MemoryConnectionOptions();
        options.Store ??= new MemoryStore();
        _options = options;
    }

    public IDatasource Create(object? options = null) =>
        new MemoryDatasource(options as MemoryConnectionOptions ?? new MemoryConnectionOptions());

    MemoryConnection NewConnection() => new(_options);
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(NewConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new MemoryCommand(connection as MemoryConnection ?? NewConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new MemoryRepository(connection as MemoryConnection ?? NewConnection());
}
