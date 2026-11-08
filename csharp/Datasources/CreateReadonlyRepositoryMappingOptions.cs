using System.Collections;
using System.Reflection;

namespace Datasources;

public static class CreateReadonlyRepositoryMappingOptions
{
    public static ReadonlyRepositoryMappingOptions<T> FromType<T>(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        FromType<T>(typeof(T), types);

    public static ReadonlyRepositoryMappingOptions<T> FromType<T>(
        Type type,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        new()
        {
            Entity = type.Name,
            Schema = SchemaOf(type),
            RowTypeConverters = types
        };

    static Schema SchemaOf(Type type)
    {
        if (typeof(IDictionary).IsAssignableFrom(type))
            return new Schema(new Dictionary<string, SchemaColumn>());
        var mapper = new DeterministicTypeMapper();
        return new Schema(
            type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
                .Select(prop =>
                {
                    var underlying = Nullable.GetUnderlyingType(prop.PropertyType);
                    var mapped = (underlying ?? prop.PropertyType) == typeof(string)
                        ? "string"
                        : mapper.Map((underlying ?? prop.PropertyType).Name);
                    return mapped.Length == 0
                        ? default((string Name, SchemaColumn Column)?)
                        : (prop.Name, new SchemaColumn(mapped, underlying != null, false));
                })
                .Where(pair => pair is not null)
                .ToDictionary(pair => pair!.Value.Name, pair => pair!.Value.Column));
    }
}
