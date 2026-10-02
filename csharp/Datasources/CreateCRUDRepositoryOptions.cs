using System.Collections;
using System.Reflection;

namespace Datasources;

public static class CreateCRUDRepositoryOptions
{
    public static CRUDRepositoryOptions<T> FromType<T>(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        FromType<T>(typeof(T), types);

    public static CRUDRepositoryOptions<T> FromType<T>(
        Type type,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new()
    {
        var read = CreateReadonlyRepositoryMappingOptions.FromType<T>(type, types);
        return new()
        {
            Ids = DefaultId(type),
            Entity = read.Entity,
            RowTypeConverters = read.RowTypeConverters
        };
    }

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
