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
        var mapping = new NameSourceDestinationMapper(options.Connection.Provider, "csharp");
        var table = options.Table ?? typeof(T).Name;
        return new(
            options.Connection,
            rowConverter,
            mapping,
            new RepositoryStatements(table, Columns()),
            options.Ids,
            options.AssignIdentityOnAdd);
    }

    public static Repository<T> Build(RepositoryBuilderOptions<T> options) => new(Parts(options));

    static string[] Columns() =>
        typeof(IDictionary).IsAssignableFrom(typeof(T))
            ? []
            : [.. typeof(T).GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
                .Select(prop => prop.Name)
                .OrderBy(name => name)];
}
