using System.Collections;
using System.Reflection;

namespace Datasources;

public static class CreateRepositoryBuilderOptions
{
    public static RepositoryBuilderOptions<T> FromType<T>(
        IConnection connection,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        FromType<T>(typeof(T), connection, types);

    public static RepositoryBuilderOptions<T> FromType<T>(
        Type type,
        IConnection connection,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        new()
        {
            Ids = DefaultId(type),
            Connection = connection,
            Types = types
        };

    static IReadOnlyList<string> DefaultId(Type type)
    {
        if (typeof(IDictionary).IsAssignableFrom(type))
            return [];
        var fields = type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
            .Select(prop => prop.Name);
        return fields.Contains("id") ? ["id"] : fields.Contains("Id") ? ["Id"] : [];
    }
}
