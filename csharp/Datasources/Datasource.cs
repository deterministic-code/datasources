using System.Collections;
using System.Reflection;

namespace Datasources;

public sealed class Datasource(
    string provider,
    IDriver driver,
    IReadOnlyDictionary<string, string?>? options = null) : IDatasource
{
    public static IDatasource Default => DatasourceFactory.Create();
    IConnection? _connection;
    public string Provider { get; } = provider;
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

    public IReadonlyRepository<T> CreateReadonlyRepository<T>(
        IConnection? connection = null,
        ReadonlyRepositoryOptions<T>? options = null) where T : new() =>
        new ReadonlyRepository<T>(connection ?? DefaultConnection(), Apply(options));

    CRUDRepositoryOptions<T> Apply<T>(CRUDRepositoryOptions<T>? options) where T : new()
    {
        var built = CreateCRUDRepositoryOptions.FromType<T>(driver.RowTypeConverters);
        var types = driver.RowTypeConverters;
        var rows = RowTypeConverter.From(types, Columns(typeof(T)));
        return options is null
            ? new()
            {
                Ids = built.Ids,
                Entity = built.Entity,
                RowTypeConverters = types,
                RowConverter = rows,
            }
            : new()
            {
                Entity = options.Entity ?? built.Entity,
                EntityMapper = options.EntityMapper,
                PropertyMappings = options.PropertyMappings,
                RowConverter = options.RowConverter ?? rows,
                ObjectConverter = options.ObjectConverter,
                ReadProperties = options.ReadProperties,
                RowTypeConverters = types,
                Ids = options.Ids,
                Write = options.Write,
            };
    }

    ReadonlyRepositoryOptions<T> Apply<T>(ReadonlyRepositoryOptions<T>? options) where T : new()
    {
        var built = CreateReadonlyRepositoryOptions.FromType<T>(driver.RowTypeConverters);
        var types = driver.RowTypeConverters;
        var rows = RowTypeConverter.From(types, Columns(typeof(T)));
        return options is null
            ? new()
            {
                Entity = built.Entity,
                RowTypeConverters = types,
                RowConverter = rows,
            }
            : new()
            {
                Entity = options.Entity ?? built.Entity,
                EntityMapper = options.EntityMapper,
                PropertyMappings = options.PropertyMappings,
                RowConverter = options.RowConverter ?? rows,
                ObjectConverter = options.ObjectConverter,
                ReadProperties = options.ReadProperties,
                RowTypeConverters = types,
            };
    }

    static IReadOnlyDictionary<string, string> Columns(Type type) =>
        typeof(IDictionary).IsAssignableFrom(type)
            ? new Dictionary<string, string>()
            : type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
                .Select(prop => (prop.Name, Type: Canonical(prop.PropertyType)))
                .Where(pair => pair.Type is not null)
                .ToDictionary(pair => pair.Name, pair => pair.Type!);

    static string? Canonical(Type type) => (Nullable.GetUnderlyingType(type) ?? type) switch
    {
        var kind when kind == typeof(Guid) => "uuid",
        var kind when kind == typeof(bool) => "bool",
        var kind when kind == typeof(DateTime) => "datetime",
        var kind when kind == typeof(decimal) => "decimal",
        var kind when kind == typeof(byte[]) => "binary",
        _ => null
    };
}
