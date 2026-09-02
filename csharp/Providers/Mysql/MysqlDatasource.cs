namespace Datasources.Mysql;

public sealed class MysqlDatasource : IDatasource, IDatasourceFactory
{
    static MysqlDatasource() => _ = MysqlTypes.All;

    public static readonly MysqlDatasource Instance = new();
    private readonly MysqlConnectionOptions _options;
    private readonly string? _table;
    public string Provider => "mysql";

    public MysqlDatasource(MysqlConnectionOptions? options = null, string? table = null)
    {
        _options = options ?? new MysqlConnectionOptions();
        _table = table;
    }

    MysqlConnection? _connection;

    public IDatasource Create(object? options = null) =>
        options is MysqlConnectionOptions typed
            ? new MysqlDatasource(typed)
            : new MysqlDatasource(new MysqlConnectionOptions
            {
                Url = DatasourceOptions.Value(options, "url")
            });

    MysqlConnection NewConnection() => new(_options);
    MysqlConnection DefaultConnection() => _connection ??= NewConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(DefaultConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new MysqlCommand(connection as MysqlConnection ?? DefaultConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new MysqlRepository(connection as MysqlConnection ?? DefaultConnection(), _table);
}
