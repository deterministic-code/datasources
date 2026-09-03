using Datasources;

namespace Datasources.Yaml;

public sealed class YamlDatasource : IDatasource, IDatasourceFactory
{
    public static readonly YamlDatasource Instance = new();
    public string Provider => "yaml";
    public IDatasource Create(object? options = null) => new YamlDatasource();
    IConnection? _connection;
    IConnection DefaultConnection() => _connection ??= new YamlConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new YamlCommand(connection ?? DefaultConnection());
    public ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
