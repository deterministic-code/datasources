using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresDatasource : IDatasource, IDatasourceFactory
{
    public static readonly PostgresDatasource Factory = new();
    public string Provider => "postgres";
    public IDatasource Create(object? options = null) => new PostgresDatasource();
    IConnection NewConnection() => new PostgresConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new PostgresCommand(connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
