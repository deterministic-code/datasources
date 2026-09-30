using System.Collections;
using System.Reflection;

namespace Datasources;

public static class RepositoryBuilder<T> where T : new()
{
    public static BuiltRepository<T> Parts(IConnection connection, ReadonlyRepositoryOptions<T> options)
    {
        var (ids, insert, update, delete) = options switch
        {
            CRUDRepositoryOptions<T> crud => (crud.Ids, crud.Insert, crud.Update, crud.Delete),
            _ => (Array.Empty<string>(), null, null, null)
        };
        var columns = (options.ReadProperties is { Count: > 0 } listed ? listed : Columns())
            .Select(options.ColumnMappings.Map)
            .OrderBy(name => name, StringComparer.Ordinal)
            .ToArray();
        return new(
            connection,
            options.RowConverter,
            options.ObjectConverter ?? new MetaDataRowTypeConverter<T>(),
            options.RowsMapper,
            options.ColumnMappings,
            new RepositoryStatements(
                options.EntityMapper.Map(options.Entity ?? typeof(T).Name),
                columns,
                options.Select,
                insert,
                update,
                delete),
            ids);
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

    static bool IsColumn(Type type) =>
        (Nullable.GetUnderlyingType(type) ?? type) is { } kind
        && (kind == typeof(string) || kind == typeof(byte[])
            || kind.IsValueType && !typeof(IEnumerable).IsAssignableFrom(kind));
}
