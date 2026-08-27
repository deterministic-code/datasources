using Datasources;

namespace Datasources.Oracle;

public sealed class OracleDatasource : IDatasource, IDatasourceFactory
{
    public static readonly OracleDatasource Factory = new();
    public string Provider => "oracle";
    public IDatasource Create(object? options = null) => new OracleDatasource();
    IConnection NewConnection() => new OracleConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new OracleCommand(connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
