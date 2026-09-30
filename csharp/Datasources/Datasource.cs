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
        var built = CreateCRUDRepositoryOptions.FromType<T>(driver.RowTypeConverters);
        var types = driver.RowTypeConverters;
        var rows = driver.RowConverter(Columns(typeof(T)));
        return options switch
        {
            null => new()
            {
                Ids = built.Ids,
                Entity = built.Entity,
                RowTypeConverters = types,
                RowConverter = rows,
            },
            CRUDRepositoryOptions<T> crud => new()
            {
                Entity = crud.Entity ?? built.Entity,
                EntityMapper = crud.EntityMapper,
                ColumnMappings = crud.ColumnMappings,
                Select = crud.Select,
                RowsMapper = crud.RowsMapper,
                RowConverter = crud.RowConverter ?? rows,
                ObjectConverter = crud.ObjectConverter,
                ReadProperties = crud.ReadProperties,
                RowTypeConverters = types,
                Ids = crud.Ids,
                Insert = crud.Insert,
                Update = crud.Update,
                Delete = crud.Delete,
                Return = crud.Return,
                Write = crud.Write,
            },
            { } read => new()
            {
                Entity = read.Entity ?? built.Entity,
                EntityMapper = read.EntityMapper,
                ColumnMappings = read.ColumnMappings,
                Select = read.Select,
                RowsMapper = read.RowsMapper,
                RowConverter = read.RowConverter ?? rows,
                ObjectConverter = read.ObjectConverter,
                ReadProperties = read.ReadProperties,
                RowTypeConverters = types,
                Ids = built.Ids,
            }
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

