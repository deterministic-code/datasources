using System.Collections;
using System.Reflection;

namespace Datasources;

public static class TypeConverter
{
    static readonly Dictionary<string, ProviderTypes> Registry = [];

    public static void Use(string source, ProviderTypes types) => Registry[source] = types;
    public static void Reset() => Registry.Clear();

    public static ProviderTypes Table(string source) =>
        Registry.TryGetValue(source, out var table)
            ? table
            : throw new DatasourceException($"Provider \"{source}\" is not registered");

    public static TypeMapping<T> Mapping<T>(
        string source,
        IReadOnlyDictionary<string, IFieldConverter>? converters = null,
        string? name = null) where T : new()
    {
        var table = Table(source);
        IFieldConverter ConverterFor(string key, Type? type = null) =>
            converters is not null && converters.TryGetValue(key, out var overrideConverter) ? overrideConverter
            : type is not null && table.Types.TryGetValue(FieldConverters.SqlType(type), out var typed) ? typed
            : FieldConverters.Identity;
        return new TypeMapping<T>
        {
            Name = name ?? typeof(T).Name,
            Source = source,
            To = row =>
            {
                var target = new T();
                if (target is IDictionary<string, Value> dict)
                {
                    foreach (var (key, value) in row)
                        dict[key] = ConverterFor(key).To(value);
                    return target;
                }
                foreach (var (key, value) in row)
                {
                    var prop = typeof(T).GetProperty(key);
                    if (prop is null || !prop.CanWrite || prop.GetIndexParameters().Length > 0) continue;
                    prop.SetValue(target, FieldConverters.Assign(ConverterFor(key, prop.PropertyType).To(value), prop.PropertyType));
                }
                return target;
            },
            From = obj =>
            {
                var result = new Dictionary<string, Value>();
                if (obj is IDictionary<string, Value> dict)
                {
                    foreach (var (key, value) in dict)
                        result[key] = ConverterFor(key).From(value);
                    return result;
                }
                foreach (var prop in Props(typeof(T)))
                    result[prop.Name] = ConverterFor(prop.Name, prop.PropertyType).From(Value.From(prop.GetValue(obj)));
                return result;
            }
        };
    }

    internal static PropertyInfo[] Props(Type type) =>
        typeof(IDictionary).IsAssignableFrom(type)
            ? []
            : [.. type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)];
}
