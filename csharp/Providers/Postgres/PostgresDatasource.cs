using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresDatasource : IDatasource, IDatasourceFactory
{
    static PostgresDatasource() => _ = PostgresTypes.All;

    public static readonly PostgresDatasource Instance = new();
    private readonly PostgresConnectionOptions _options;
    private readonly string? _table;
    public string Provider => "postgres";

    public PostgresDatasource(PostgresConnectionOptions? options = null, string? table = null)
    {
        _options = options ?? new PostgresConnectionOptions();
        _table = table;
    }

    PostgresConnection? _connection;

    public IDatasource Create(object? options = null) =>
        options is PostgresConnectionOptions typed
            ? new PostgresDatasource(typed)
            : new PostgresDatasource(new PostgresConnectionOptions
            {
                Url = DatasourceOptions.Value(options, "url"),
                Schema = DatasourceOptions.Value(options, "schema")
            });

    PostgresConnection NewConnection() => new(_options);
    PostgresConnection DefaultConnection() => _connection ??= NewConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(DefaultConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new PostgresCommand(connection as PostgresConnection ?? DefaultConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new Repository<Dictionary<string, Value>>(
            new RepositoryBuilderOptions<Dictionary<string, Value>>
            {
                Connection = connection as PostgresConnection ?? DefaultConnection(),
                Entity = _table ?? "items",
                Mapping = new RowTypeConverter<Dictionary<string, Value>>(),
            });
}
