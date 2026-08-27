using Datasources;

namespace Datasources.Json;

public sealed class JsonDatasource : IDatasource, IDatasourceFactory
{
    public static readonly JsonDatasource Factory = new();
    private readonly JsonConnectionOptions _options;
    public string Provider => "json";
    public JsonDatasource(JsonConnectionOptions? options = null) => _options = options ?? new JsonConnectionOptions();
    public IDatasource Create(object? options = null) =>
        new JsonDatasource(options as JsonConnectionOptions ?? new JsonConnectionOptions());
    JsonConnection NewConnection() => new(_options);
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new JsonCommand(connection as JsonConnection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new JsonRepository(connection as JsonConnection ?? NewConnection());
}
