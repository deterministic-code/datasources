using System.Collections;
using System.Reflection;

namespace Datasources;

public static class CreateRepositoryBuilderOptions
{
    public static RepositoryBuilderOptions<T> FromType<T>(IConnection connection)
        where T : new() =>
        FromType<T>(typeof(T), connection);

    public static RepositoryBuilderOptions<T> FromType<T>(Type type, IConnection connection)
        where T : new() =>
        new()
        {
            Ids = DefaultId(type),
            Connection = connection
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
