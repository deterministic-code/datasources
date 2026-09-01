using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteDatasource : IDatasource, IDatasourceFactory
{
    static SqliteDatasource() => _ = SqliteTypes.All;

    public static readonly SqliteDatasource Instance = new();
    private readonly SqliteConnectionOptions _options;
    private readonly string? _table;
    public string Provider => "sqlite";

    public SqliteDatasource(SqliteConnectionOptions? options = null, string? table = null)
    {
        _options = options ?? new SqliteConnectionOptions();
        _table = table;
    }

    SqliteConnection? _connection;

    public IDatasource Create(object? options = null) =>
        options is SqliteConnectionOptions typed
            ? new SqliteDatasource(typed)
            : new SqliteDatasource(new SqliteConnectionOptions
            {
                Filename = DatasourceOptions.Value(options, "filename")
            });

    SqliteConnection NewConnection() => new(_options);
    SqliteConnection DefaultConnection() => _connection ??= NewConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(DefaultConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new SqliteCommand(connection as SqliteConnection ?? DefaultConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new Repository<Dictionary<string, Value>>(
            new RepositoryBuilderOptions<Dictionary<string, Value>>
            {
                Connection = connection as SqliteConnection ?? DefaultConnection(),
                Entity = _table ?? "items",
                Mapping = new RowTypeConverter<Dictionary<string, Value>>(),
            });
}
