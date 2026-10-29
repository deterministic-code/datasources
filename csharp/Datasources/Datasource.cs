using System.Collections;
using System.Dynamic;
using System.Reflection;

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
        var built = CreateCRUDRepositoryOptions.FromType<T>(driver.RowTypeConverters);
        var types = driver.RowTypeConverters;
        var schema = options?.Schema ?? SchemaOf(typeof(T));
        var rows = RowTypeConverter.From(types, schema.Columns);
        return (options ?? built) with
        {
            Entity = options?.Entity ?? built.Entity,
            Schema = schema,
            RowConverter = options?.RowConverter ?? rows,
            RowTypeConverters = types,
        };
    }

    ReadonlyRepositoryMappingOptions<T> Apply<T>(ReadonlyRepositoryMappingOptions<T>? options) where T : new()
    {
        var built = CreateReadonlyRepositoryMappingOptions.FromType<T>(driver.RowTypeConverters);
        var types = driver.RowTypeConverters;
        var schema = options?.Schema ?? SchemaOf(typeof(T));
        var rows = RowTypeConverter.From(types, schema.Columns);
        return (options ?? built) with
        {
            Entity = options?.Entity ?? built.Entity,
            Schema = schema,
            RowConverter = options?.RowConverter ?? rows,
            RowTypeConverters = types,
        };
    }

    static Schema SchemaOf(Type type)
    {
        var mapper = new DeterministicTypeMapper();
        return typeof(IDictionary).IsAssignableFrom(type)
            ? new Schema(new Dictionary<string, string>())
            : new Schema(
                type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
                    .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
                    .Select(prop => (prop.Name, Spec: mapper.Map((Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType).Name)))
                    .Where(pair => pair.Spec.Length > 0)
                    .ToDictionary(pair => pair.Name, pair => pair.Spec));
    }
}
