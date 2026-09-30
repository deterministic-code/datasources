using System.Collections;
using System.Reflection;

namespace Datasources;

public sealed class MetaDataRowTypeConverter<T> : TypeConverter<T, Row> where T : new()
{
    public Dictionary<string, TypeConverter<Value, Value>> Types { get; set; } = [];
    public Dictionary<string, TypeConverter<Value, Value>> Fields { get; set; } = [];

    public MetaDataRowTypeConverter(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? fields = null)
    {
        if (types is not null)
            Types = new(types);
        if (fields is not null)
            Fields = new(fields);
    }

    public override T Convert(Row from)
    {
        var target = new T();
        if (target is IDictionary<string, Value> dict)
        {
            foreach (var (key, value) in from)
                dict[key] = Converted(key, value, reverse: false);
            return target;
        }
        foreach (var (key, value) in from)
        {
            var prop = typeof(T).GetProperty(key);
            if (prop is null || !prop.CanWrite || prop.GetIndexParameters().Length > 0
                || !IsColumn(prop.PropertyType)) continue;
            var cell = Converted(key, value, reverse: false);
            var underlying = Nullable.GetUnderlyingType(prop.PropertyType);
            var type = underlying ?? prop.PropertyType;
            var raw = cell.ToObject();
            if (raw is null ? underlying is not null || !type.IsValueType : type.IsInstanceOfType(raw))
                prop.SetValue(target, raw);
        }
        return target;
    }

    public override Row ReverseConvert(T to) => to switch
    {
        IDictionary<string, Value> dict => new(dict.Select(pair =>
            KeyValuePair.Create(pair.Key, Converted(pair.Key, pair.Value, reverse: true)))),
        _ => new(Props(typeof(T))
            .Where(prop => IsColumn(prop.PropertyType))
            .Select(prop => (prop, Raw: prop.GetValue(to)))
            .Where(pair => pair.Raw is not null)
            .Select(pair => KeyValuePair.Create(
                pair.prop.Name,
                Converted(pair.prop.Name, Value.From(pair.Raw), reverse: true))))
    };

    Value Converted(string key, Value value, bool reverse) =>
        Converter(key) switch
        {
            null => value,
            var converter when reverse => converter.ReverseConvert(value),
            var converter => converter.Convert(value)
        };

    TypeConverter<Value, Value>? Converter(string key) =>
        Fields.TryGetValue(key, out var field) ? field : null;

    static bool IsColumn(Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        return type == typeof(string) || type == typeof(byte[])
            || type.IsValueType && !typeof(IEnumerable).IsAssignableFrom(type);
    }

    static PropertyInfo[] Props(Type type) =>
        typeof(IDictionary).IsAssignableFrom(type)
            ? []
            : [.. type.GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)];
}
