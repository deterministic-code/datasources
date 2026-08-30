using System.Collections;
using System.Globalization;
using System.Reflection;

namespace Datasources;

public sealed class RowConverter
{
    public T Convert<T>(Row row) where T : new()
    {
        var target = new T();
        if (target is IDictionary<string, Value> dict)
        {
            foreach (var (key, value) in row)
                dict[key] = value;
            return target;
        }
        foreach (var (key, value) in row)
        {
            var prop = typeof(T).GetProperty(key);
            if (prop is null || !prop.CanWrite || prop.GetIndexParameters().Length > 0) continue;
            prop.SetValue(target, Assign(value, prop.PropertyType));
        }
        return target;
    }

    public Row ReverseConvert<T>(T obj)
    {
        if (obj is IDictionary<string, Value> dict)
            return new Row(dict);
        var result = new Row();
        foreach (var prop in Props(typeof(T)))
            result[prop.Name] = Value.From(prop.GetValue(obj));
        return result;
    }

    internal static PropertyInfo[] Props(Type type) =>
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
            string text when type == typeof(DateTime) =>
                DateTime.Parse(text, CultureInfo.InvariantCulture, DateTimeStyles.RoundtripKind),
            string text when type == typeof(Guid) => Guid.Parse(text),
            string text when type == typeof(byte[]) => System.Convert.FromBase64String(text),
            string text when type == typeof(decimal) => decimal.Parse(text, CultureInfo.InvariantCulture),
            long n when type == typeof(bool) => n != 0,
            _ => System.Convert.ChangeType(raw, type, CultureInfo.InvariantCulture)
        };
    }
}
