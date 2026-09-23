using System.Collections;
using System.Reflection;

namespace Datasources;

public static class CreateRepositoryOptions
{
    public static RepositoryOptions<T> FromType<T>(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        FromType<T>(typeof(T), types);

    public static RepositoryOptions<T> FromType<T>(
        Type type,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        new()
        {
            Ids = DefaultId(type),
            Entity = type.Name,
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
