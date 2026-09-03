using System.Collections;
using System.Globalization;
using System.Reflection;

namespace Datasources;

public sealed class RowTypeConverter<T> : TypeConverter<T, Row> where T : new()
{
    public Dictionary<string, TypeConverter<Value, Value>> Types { get; set; } = [];
    public Dictionary<string, TypeConverter<Value, Value>> Fields { get; set; } = [];

    public RowTypeConverter(
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
        var native = Apply(from, reverse: false);
        var target = new T();
        if (target is IDictionary<string, Value> dict)
        {
            foreach (var (key, value) in native)
                dict[key] = value;
            return target;
        }
        foreach (var (key, value) in native)
        {
            var prop = typeof(T).GetProperty(key);
            if (prop is null || !prop.CanWrite || prop.GetIndexParameters().Length > 0
                || !IsColumn(prop.PropertyType)) continue;
            prop.SetValue(target, Assign(value, prop.PropertyType));
        }
        return target;
    }

    public override Row ReverseConvert(T to) => to switch
    {
        IDictionary<string, Value> dict => Apply(new Row(dict), reverse: true),
        _ => Apply(new Row(Props(typeof(T))
            .Where(prop => IsColumn(prop.PropertyType))
            .Select(prop => (prop.Name, Value: prop.GetValue(to)))
            .Where(pair => pair.Value is not null)
            .Select(pair => KeyValuePair.Create(pair.Name, Value.From(pair.Value)))), reverse: true)
    };

    Row Apply(Row row, bool reverse) =>
        new(row.Select(pair => KeyValuePair.Create(pair.Key, Cell(pair.Key, pair.Value, reverse))));

    Value Cell(string key, Value value, bool reverse = false)
    {
        var converter = Fields.TryGetValue(key, out var field) ? field : null;
        return converter is null ? value
            : reverse ? converter.ReverseConvert(value) : converter.Convert(value);
    }

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

    static object? Assign(Value value, Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        var raw = value.ToObject();
        return raw switch
        {
            null => null,
            _ when type.IsInstanceOfType(raw) => raw,
            string text when type == typeof(Guid) => Guid.Parse(text),
            string text when type == typeof(DateTime) => DateTime.Parse(text, CultureInfo.InvariantCulture, DateTimeStyles.RoundtripKind),
            _ => System.Convert.ChangeType(raw, type, CultureInfo.InvariantCulture)
        };
    }
}
