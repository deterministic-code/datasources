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
        var names = options.ReadProperties is { Count: > 0 } listed ? listed : Columns();
        var columns = names
            .Select(options.ColumnMappings.Map)
            .OrderBy(name => name, StringComparer.Ordinal)
            .ToArray();
        var converter = options.ObjectConverter ?? new MetaDataRowTypeConverter<T>();
        if (converter is MetaDataRowTypeConverter<T> meta)
        {
            meta.References.Clear();
            meta.ReferenceColumns = options.ColumnMappings;
            foreach (var name in names)
                if (typeof(T).GetProperty(name) is { } prop && IsReference(prop.PropertyType))
                    meta.References.Add(name);
        }
        return new(
            connection,
            options.RowConverter,
            converter,
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
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0 && Included(prop.PropertyType))
                .Select(prop => prop.Name)
                .OrderBy(name => name)];

    static bool Included(Type type) => IsColumn(type) || IsReference(type);

    static bool IsColumn(Type type) =>
        (Nullable.GetUnderlyingType(type) ?? type) is { } kind
        && (kind == typeof(string) || kind == typeof(byte[])
            || kind.IsValueType && !typeof(IEnumerable).IsAssignableFrom(kind));

    static bool IsReference(Type type) =>
        (Nullable.GetUnderlyingType(type) ?? type) is { } kind
        && kind.IsClass && kind != typeof(string) && !typeof(IEnumerable).IsAssignableFrom(kind);
}
