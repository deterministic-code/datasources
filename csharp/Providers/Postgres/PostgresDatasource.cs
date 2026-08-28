namespace Datasources.Postgres;

public sealed class PostgresDatasource : IDatasource, IDatasourceFactory
{
    public static readonly PostgresDatasource Factory = new();
    private readonly PostgresConnectionOptions _options;
    private readonly string? _table;
    public string Provider => "postgres";

    public PostgresDatasource(PostgresConnectionOptions? options = null, string? table = null)
    {
        _options = options ?? new PostgresConnectionOptions();
        _table = table;
    }

    public IDatasource Create(object? options = null) =>
        new PostgresDatasource(options as PostgresConnectionOptions ?? new PostgresConnectionOptions());

    PostgresConnection NewConnection() => new(_options);
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(NewConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new PostgresCommand(connection as PostgresConnection ?? NewConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new PostgresRepository(connection as PostgresConnection ?? NewConnection(), _table);
}
