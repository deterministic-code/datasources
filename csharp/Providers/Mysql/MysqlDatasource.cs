namespace Datasources.Mysql;

public sealed class MysqlDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MysqlDatasource Factory = new();
    private readonly MysqlConnectionOptions _options;
    private readonly string? _table;
    public string Provider => "mysql";

    public MysqlDatasource(MysqlConnectionOptions? options = null, string? table = null)
    {
        _options = options ?? new MysqlConnectionOptions();
        _table = table;
    }

    public IDatasource Create(object? options = null) =>
        new MysqlDatasource(options as MysqlConnectionOptions ?? new MysqlConnectionOptions());

    MysqlConnection NewConnection() => new(_options);
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(NewConnection());

    public ICommand CreateCommand(IConnection? connection = null) =>
        new MysqlCommand(connection as MysqlConnection ?? NewConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (IRepository<T>)(object)new MysqlRepository(connection as MysqlConnection ?? NewConnection(), _table);
}
