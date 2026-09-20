using System.Collections;
using System.Reflection;

namespace Datasources;

public static class RepositoryBuilder<T> where T : new()
{
    public static BuiltRepository<T> Parts(RepositoryBuilderOptions<T> options)
    {
        var types = DefaultTypeConverters.For(options.Connection.Provider);
        var rowConverter = options.Mapping ?? new RowTypeConverter<T>(types, options.Fields);
        if (rowConverter is RowTypeConverter<T> row && row.Types.Count == 0)
            foreach (var (key, converter) in types)
                row.Types[key] = converter;
        var names = new TransformMapper<string, string>(static name => name, static name => name);
        var statementMapper = options.StatementMapper ?? new StatementMapper<T>
        {
            Entity = names,
            Columns = names,
            Rows = options.Mapper ?? new TransformMapper<T, T>(static value => value, static value => value)
        };
        var entity = statementMapper.Entity.Map(options.Entity ?? typeof(T).Name);
        var columns = (options.Columns is { Count: > 0 } listed ? listed : Columns())
            .Select(statementMapper.Columns.Map)
            .OrderBy(name => name, StringComparer.Ordinal)
            .ToArray();
        return new(
            options.Connection,
            rowConverter,
            statementMapper,
            new RepositoryStatements(
                entity,
                columns,
                statementMapper.Query,
                statementMapper.Insert,
                statementMapper.Update,
                statementMapper.Delete),
            options.Ids,
            options.AssignIdentityOnCreate);
    }

    public static Repository<T> Build(RepositoryBuilderOptions<T> options) => new(Parts(options));

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
