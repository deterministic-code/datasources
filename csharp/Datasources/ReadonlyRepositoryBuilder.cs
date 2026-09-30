using System.Collections;
using System.Reflection;

namespace Datasources;

public class ReadonlyRepositoryBuilder<T> where T : new()
{
    public BuiltRepository<T> Parts(IConnection connection, ReadonlyRepositoryOptions<T> options)
    {
        var names = options.ReadProperties is { Count: > 0 } listed ? listed : Columns();
        var columns = names
            .Select(options.PropertyMappings.Map)
            .OrderBy(name => name, StringComparer.Ordinal)
            .ToArray();
        var converter = options.ObjectConverter ?? new MetaDataRowTypeConverter<T>();
        if (converter is MetaDataRowTypeConverter<T> meta)
        {
            meta.References.Clear();
            meta.ReferenceColumns = options.PropertyMappings;
            foreach (var name in names)
                if (typeof(T).GetProperty(name) is { } prop && IsReference(prop.PropertyType))
                    meta.References.Add(name);
        }
        return new(
            connection,
            options.RowConverter,
            converter,
            options.PropertyMappings,
            options.EntityMapper,
            Statements(options.EntityMapper.Map(options.Entity ?? typeof(T).Name), columns),
            Ids(options));
    }

    protected virtual IReadOnlyList<string> Ids(ReadonlyRepositoryOptions<T> options) => [];

    protected virtual ReadonlyRepositoryStatements Statements(string table, IReadOnlyList<string> columns) =>
        new(table, columns);

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
