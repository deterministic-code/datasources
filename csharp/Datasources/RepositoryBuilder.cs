using System.Collections;
using System.Reflection;

namespace Datasources;

public static class RepositoryBuilder<T> where T : new()
{
    public static BuiltRepository<T> Parts(IConnection connection, RepositoryOptions<T> options)
    {
        var types = options.Types ?? new Dictionary<string, TypeConverter<Value, Value>>();
        var rowConverter = options.Mapping ?? new RowTypeConverter<T>(types, options.Fields);
        if (rowConverter is RowTypeConverter<T> row && row.Types.Count == 0)
            foreach (var (key, converter) in types)
                row.Types[key] = converter;
        var names = new TransformMapper<string, string>(static name => name, static name => name);
        var crudMapper = options.CRUDMapper ?? new CRUDMapper<T>
        {
            Entity = names,
            Columns = names,
            Rows = options.Mapper ?? new TransformMapper<T, T>(static value => value, static value => value)
        };
        var entity = crudMapper.Entity.Map(options.Entity ?? typeof(T).Name);
        var columns = (options.Columns is { Count: > 0 } listed ? listed : Columns())
            .Select(crudMapper.Columns.Map)
            .OrderBy(name => name, StringComparer.Ordinal)
            .ToArray();
        return new(
            connection,
            rowConverter,
            crudMapper,
            new RepositoryStatements(
                entity,
                columns,
                crudMapper.Query,
                crudMapper.Insert,
                crudMapper.Update,
                crudMapper.Delete),
            options.Ids);
    }

    public static CRUDRepository<T> Build(IConnection connection, RepositoryOptions<T> options) =>
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
