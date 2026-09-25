using System.Collections;
using System.Reflection;

namespace Datasources;

public static class RepositoryBuilder<T> where T : new()
{
    public static BuiltRepository<T> Parts(IConnection connection, ReadonlyRepositoryOptions<T> options)
    {
        var types = options.RowTypeConverters ?? new Dictionary<string, TypeConverter<Value, Value>>();
        var rowConverter = options.RowConverter ?? new RowTypeConverter<T>(types);
        if (rowConverter is RowTypeConverter<T> { Types.Count: 0 } row)
            foreach (var (key, converter) in types)
                row.Types[key] = converter;
        var entity = options.EntityMapper.Map(options.Entity ?? typeof(T).Name);
        var columns = (options.ReadProperties is { Count: > 0 } listed ? listed : Columns())
            .Select(options.ColumnMappings.Map)
            .OrderBy(name => name, StringComparer.Ordinal)
            .ToArray();
        var (ids, insert, update, delete, create) = options switch
        {
            CRUDRepositoryOptions<T> crud => (
                crud.Ids,
                crud.Insert,
                crud.Update,
                crud.Delete,
                crud.Create ?? CRUDRepositoryOptions<T>.Identity(crud.Ids)),
            _ => (Array.Empty<string>(), null, null, null, null)
        };
        return new(
            connection,
            rowConverter,
            options.RowsMapper,
            options.ColumnMappings,
            new RepositoryStatements(entity, columns, options.Query, insert, update, delete),
            ids,
            create);
    }

    public static CRUDRepository<T> Build(IConnection connection, CRUDRepositoryOptions<T> options) =>
        new(connection, options);

    static string[] Columns() =>
        typeof(IDictionary).IsAssignableFrom(typeof(T))
            ? []
            : [.. typeof(T).GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0 && IsColumn(prop.PropertyType))
                .Select(prop => prop.Name)
                .OrderBy(name => name)];

    static bool IsColumn(Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        return type == typeof(string) || type == typeof(byte[])
            || type.IsValueType && !typeof(IEnumerable).IsAssignableFrom(type);
    }
}
