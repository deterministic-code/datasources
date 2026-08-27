using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteDatasource : IDatasource, IDatasourceFactory
{
    public static readonly SqliteDatasource Factory = new(new SqliteConnectionOptions { Driver = new ScriptedDriver() });
    private readonly SqliteConnectionOptions _options;
    private readonly string? _table;
    public string Provider => "sqlite";

    public SqliteDatasource(SqliteConnectionOptions? options = null, string? table = null)
    {
        _options = options ?? new SqliteConnectionOptions();
        _table = table;
    }

    public IDatasource Create(object? options = null) =>
        new SqliteDatasource(options as SqliteConnectionOptions ?? new SqliteConnectionOptions());

    SqliteConnection NewConnection() => new(_options);
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(NewConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new SqliteCommand(connection as SqliteConnection ?? NewConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new SqliteRepository(connection as SqliteConnection ?? NewConnection(), _table);
}
