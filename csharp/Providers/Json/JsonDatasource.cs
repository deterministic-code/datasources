using Datasources;

namespace Datasources.Json;

public sealed class JsonDatasource : IDatasource, IDatasourceFactory
{
    public static readonly JsonDatasource Factory = new();
    private readonly JsonConnectionOptions _options;
    public string Provider => "json";
    public JsonDatasource(JsonConnectionOptions? options = null) => _options = options ?? new JsonConnectionOptions();
    JsonConnection? _connection;
    public IDatasource Create(object? options = null) =>
        options is JsonConnectionOptions typed
            ? new JsonDatasource(typed)
            : new JsonDatasource(new JsonConnectionOptions
            {
                Path = DatasourceOptions.Value(options, "path") ?? ""
            });
    JsonConnection NewConnection() => new(_options);
    JsonConnection DefaultConnection() => _connection ??= NewConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new JsonCommand(connection as JsonConnection ?? DefaultConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new JsonRepository(connection as JsonConnection ?? DefaultConnection());
}
