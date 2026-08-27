using Datasources;

namespace Datasources.Yaml;

public sealed class YamlConnection : StubConnection
{
    public YamlConnection() : base("yaml") { }
}

public sealed class YamlCommand : StubCommand
{
    public YamlCommand(IConnection connection) : base("yaml", connection) { }
}

public sealed class YamlDatasource : IDatasource, IDatasourceFactory
{
    public static readonly YamlDatasource Factory = new();
    public string Provider => "yaml";
    public IDatasource Create(object? options = null) => new YamlDatasource();
    IConnection NewConnection() => new YamlConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new YamlCommand(connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
