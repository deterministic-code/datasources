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
    public ICRUDRepository<T> CreateRepository<T>(
        IConnection? connection = null,
        CRUDRepositoryOptions<T>? options = null) where T : new() =>
        new CRUDRepository<T>(connection ?? DefaultConnection(), Apply(options));

    public IReadonlyRepository<T> CreateReadonlyRepository<T>(
        IConnection? connection = null,
        ReadonlyRepositoryOptions<T>? options = null) where T : new() =>
        new ReadonlyRepository<T>(connection ?? DefaultConnection(), Apply(options));

    CRUDRepositoryOptions<T> Apply<T>(ReadonlyRepositoryOptions<T>? options) where T : new()
    {
        var built = CreateRepositoryOptions.FromType<T>(driver.RowTypeConverters);
        var types = driver.RowTypeConverters;
        return options switch
        {
            null => built,
            CRUDRepositoryOptions<T> crud => new()
            {
                Entity = crud.Entity ?? built.Entity,
                EntityMapper = crud.EntityMapper,
                ColumnMappings = crud.ColumnMappings,
                Query = crud.Query,
                RowsMapper = crud.RowsMapper,
                Result = crud.Result,
                RowConverter = crud.RowConverter,
                ReadProperties = crud.ReadProperties,
                RowTypeConverters = types,
                Ids = crud.Ids,
                Insert = crud.Insert,
                Update = crud.Update,
                Delete = crud.Delete,
                Return = crud.Return,
                Create = crud.Create,
                Write = crud.Write,
            },
            { } read => new()
            {
                Entity = read.Entity ?? built.Entity,
                EntityMapper = read.EntityMapper,
                ColumnMappings = read.ColumnMappings,
                Query = read.Query,
                RowsMapper = read.RowsMapper,
                Result = read.Result,
                RowConverter = read.RowConverter,
                ReadProperties = read.ReadProperties,
                RowTypeConverters = types,
                Ids = built.Ids,
                Create = built.Create,
            }
        };
    }
}

