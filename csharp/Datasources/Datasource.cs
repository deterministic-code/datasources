namespace Datasources;

public sealed class Datasource(
    string provider,
    IDriver driver,
    IReadOnlyDictionary<string, string?>? options = null) : IDatasource
{
    IConnection? _connection;
    public string Provider { get; } = provider;
    IConnection DefaultConnection() => _connection ??= driver.Connection(options);
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        driver.Command(connection ?? DefaultConnection());
    public ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null) where T : new() =>
        new CRUDRepository<T>(connection ?? DefaultConnection(), CreateRepositoryOptions.FromType<T>(driver.RowTypeConverters));

    public IReadonlyRepository<T> CreateReadonlyRepository<T>(IConnection? connection = null) where T : new() =>
        new ReadonlyRepository<T>(connection ?? DefaultConnection(), CreateRepositoryOptions.FromType<T>(driver.RowTypeConverters));
}

