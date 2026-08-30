using System.Collections;
using System.Globalization;
using System.Reflection;

namespace Datasources;

public sealed class RowConverter<T> : TypeConverter<T, Row> where T : new()
{
    public IReadOnlyDictionary<Type, TypeConverter<object?, Value>> Types { get; }
    public IReadOnlyDictionary<string, TypeConverter<object?, Value>> Fields { get; }

    public RowConverter(
        IReadOnlyDictionary<Type, TypeConverter<object?, Value>>? types = null,
        IReadOnlyDictionary<string, TypeConverter<object?, Value>>? fields = null)
    {
        Types = types ?? new Dictionary<Type, TypeConverter<object?, Value>>();
        Fields = fields ?? new Dictionary<string, TypeConverter<object?, Value>>();
    }

    public static IReadOnlyList<string> Columns =>
        [.. Props(typeof(T)).Select(prop => prop.Name)];

    public override T Convert(Row from)
    {
        var target = new T();
        if (target is IDictionary<string, Value> dict)
        {
            foreach (var (key, value) in from)
                dict[key] = Fields.TryGetValue(key, out var converter)
                    ? Value.From(converter.Convert(value))
                    : value;
            return target;
        }
        foreach (var (key, value) in from)
        {
            var prop = typeof(T).GetProperty(key);
            if (prop is null || !prop.CanWrite || prop.GetIndexParameters().Length > 0) continue;
            prop.SetValue(target, Assign(key, value, prop.PropertyType));
        }
        return target;
    }

    public override Row ReverseConvert(T to) => to switch
    {
        IDictionary<string, Value> dict => new Row(dict.Select(pair =>
            KeyValuePair.Create(pair.Key, Reverse(pair.Key, null, pair.Value)))),
        _ => new Row(Props(typeof(T)).Select(prop =>
            KeyValuePair.Create(prop.Name, Reverse(prop.Name, prop.PropertyType, prop.GetValue(to)))))
    };

    static PropertyInfo[] Props(Type type) =>
        typeof(IDictionary).IsAssignableFrom(type)
            ? []
            : [.. type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)];

    object? Assign(string key, Value value, Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        var converter = Pick(key, type);
        if (converter is not null)
            return converter.Convert(value);
        var raw = value.ToObject();
        return raw switch
        {
            null => null,
            _ when type.IsInstanceOfType(raw) => raw,
            _ => System.Convert.ChangeType(raw, type, CultureInfo.InvariantCulture)
        };
    }

    Value Reverse(string key, Type? type, object? raw)
    {
        type = type is null ? null : Nullable.GetUnderlyingType(type) ?? type;
        var converter = Pick(key, type);
        return converter?.ReverseConvert(raw is Value value ? value.ToObject() : raw)
            ?? (raw is Value existing ? existing : Value.From(raw));
    }

    TypeConverter<object?, Value>? Pick(string key, Type? type) =>
        Fields.TryGetValue(key, out var field) ? field
        : type is not null && Types.TryGetValue(type, out var mapped) ? mapped
        : null;
}
