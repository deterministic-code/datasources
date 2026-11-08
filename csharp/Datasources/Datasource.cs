using System.Dynamic;

namespace Datasources;

public sealed class Datasource(
    string provider,
    IDriver driver,
    IReadOnlyDictionary<string, string?>? options = null) : DynamicObject, IDatasource
{
    public static IDatasource Default => DatasourceFactory.Create();
    IConnection? _connection;
    public string Provider { get; } = provider;
    public RepositoryFactory Repositories { get; } = new();
    IConnection DefaultConnection() => _connection ??= driver.Connection(options);
    public Task Close()
    {
        var connection = _connection;
        _connection = null;
        return connection?.Close() ?? Task.CompletedTask;
    }
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public async Task<ICommand> CreateCommand(IConnection? connection = null)
    {
        if (connection is null)
        {
            connection = DefaultConnection();
            if (connection.IsOpen)
                throw new DatasourceException("Connection is already open");
            await connection.Open();
        }
        return driver.Command(connection);
    }
    public ICRUDRepository<T> CreateRepository<T>(
        IConnection? connection = null,
        CRUDRepositoryOptions<T>? options = null) where T : new() =>
        new CRUDRepository<T>(connection ?? DefaultConnection(), Apply(options));

    public ICRUDRepository<T> CreateRepository<T>(string typeName, IConnection? connection = null) =>
        Repositories.Create<ICRUDRepository<T>>(typeName, connection ?? DefaultConnection());

    public IReadonlyRepository<T> CreateReadonlyRepository<T>(
        IConnection? connection = null,
        ReadonlyRepositoryMappingOptions<T>? options = null) where T : new() =>
        new ReadonlyRepository<T>(connection ?? DefaultConnection(), Apply(options));

    public IReadonlyRepository<T> CreateReadonlyRepository<T>(string typeName, IConnection? connection = null) =>
        Repositories.CreateReadonly<IReadonlyRepository<T>>(typeName, connection ?? DefaultConnection());

    public override bool TryGetMember(GetMemberBinder binder, out object? result)
    {
        result = Repositories.Access(binder.Name, DefaultConnection()).Create();
        return true;
    }

    CRUDRepositoryOptions<T> Apply<T>(CRUDRepositoryOptions<T>? options) where T : new()
    {
        var built = CreateCRUDRepositoryOptions.FromType<T>();
        return (options ?? built) with
        {
            Entity = options?.Entity ?? built.Entity,
            Schema = options?.Schema ?? built.Schema,
        };
    }

    ReadonlyRepositoryMappingOptions<T> Apply<T>(ReadonlyRepositoryMappingOptions<T>? options) where T : new()
    {
        var built = CreateReadonlyRepositoryMappingOptions.FromType<T>();
        return (options ?? built) with
        {
            Entity = options?.Entity ?? built.Entity,
            Schema = options?.Schema ?? built.Schema,
        };
    }
}
