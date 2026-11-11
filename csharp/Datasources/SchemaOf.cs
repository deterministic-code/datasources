using System.Collections;
using System.Reflection;

namespace Datasources;

public static class SchemaOf
{
    public static Schema From<T>() => From(typeof(T));

    public static Schema From(Type type)
    {
        var mapper = new DeterministicTypeMapper();
        return typeof(IDictionary).IsAssignableFrom(type)
            ? new Schema(new Dictionary<string, SchemaColumn>())
            : new Schema(
                type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
                    .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
                    .Select(prop =>
                    {
                        var kind = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
                        return (prop.Name, Column: new SchemaColumn(mapper.Map(kind.Name), Nullable.GetUnderlyingType(prop.PropertyType) != null));
                    })
                    .Where(pair => pair.Column.Type.Length > 0)
                    .ToDictionary(pair => pair.Name, pair => pair.Column));
    }
}
