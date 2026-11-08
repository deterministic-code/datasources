using System.Collections;
using System.Globalization;
using System.Reflection;

namespace Datasources;

public sealed class MetaDataRowTypeConverter<T> : TypeConverter<T, Row> where T : new()
{
    public Dictionary<string, TypeConverter<Value, Value>> Fields { get; set; } = [];
    public HashSet<string> References { get; } = [];
    public IMapper<string, string> ReferenceColumns { get; set; } = new IdentityMapper<string>();
    public bool ShouldCoerceId { get; set; }
    public IReadOnlyList<string> Ids { get; set; } = [];

    public MetaDataRowTypeConverter(IReadOnlyDictionary<string, TypeConverter<Value, Value>>? fields = null)
    {
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
            if (prop is null || !prop.CanWrite || prop.GetIndexParameters().Length > 0) continue;
            if (References.Contains(prop.Name))
            {
                SetReferenceId(target, prop, Converted(key, value, reverse: false), Ids);
                continue;
            }
            if (!IsColumn(prop.PropertyType)) continue;
            var cell = Converted(key, value, reverse: false);
            var underlying = Nullable.GetUnderlyingType(prop.PropertyType);
            var type = underlying ?? prop.PropertyType;
            var raw = cell.ToObject();
            if (raw is null ? underlying is not null || !type.IsValueType : type.IsInstanceOfType(raw))
                prop.SetValue(target, raw);
            else if (Coerced(raw, type) is { } coerced)
                prop.SetValue(target, coerced);
        }
        return target;
    }

    public override Row ReverseConvert(T to) => to switch
    {
        IDictionary<string, Value> dict => new(dict.Select(pair =>
            KeyValuePair.Create(pair.Key, Converted(pair.Key, pair.Value, reverse: true)))),
        _ => new(Props(typeof(T)).SelectMany(prop => Cells(to, prop)))
    };

    IEnumerable<KeyValuePair<string, Value>> Cells(T source, PropertyInfo prop)
    {
        if (References.Contains(prop.Name))
        {
            if (ReferenceId(prop.GetValue(source), Ids) is { } id)
                yield return new(ReferenceColumns.Map(prop.Name), Converted(prop.Name, id, reverse: true));
            yield break;
        }
        if (!IsColumn(prop.PropertyType)) yield break;
        yield return prop.GetValue(source) is { } raw
            ? new(prop.Name, Converted(prop.Name, Value.From(raw), reverse: true))
            : new(prop.Name, Value.Null.Instance);
    }

    static void SetReferenceId(T target, PropertyInfo prop, Value cell, IReadOnlyList<string> ids)
    {
        if (cell.ToObject() is not { } raw) return;
        var type = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
        var current = prop.GetValue(target) ?? Activator.CreateInstance(type);
        if (current is null) return;
        var id = ids.Select(type.GetProperty).FirstOrDefault(found => found is not null)
            ?? type.GetProperty("Id") ?? type.GetProperty("id");
        if (id is null) return;
        var idType = Nullable.GetUnderlyingType(id.PropertyType) ?? id.PropertyType;
        id.SetValue(current, idType.IsInstanceOfType(raw) ? raw : System.Convert.ChangeType(raw, idType));
        prop.SetValue(target, current);
    }

    static Value? ReferenceId(object? value, IReadOnlyList<string> ids)
    {
        if (value is null) return null;
        var id = ids.Select(value.GetType().GetProperty).FirstOrDefault(found => found is not null)
            ?? value.GetType().GetProperty("Id") ?? value.GetType().GetProperty("id");
        return id?.GetValue(value) is { } raw ? Value.From(raw) : null;
    }

    Value Converted(string key, Value value, bool reverse) =>
        Converter(key) switch
        {
            null => value,
            var converter when reverse => converter.ReverseConvert(value),
            var converter => converter.Convert(value)
        };

    TypeConverter<Value, Value>? Converter(string key) =>
        Fields.TryGetValue(key, out var field) ? field : null;

    object? Coerced(object? raw, Type type)
    {
        if (raw is null) return null;
        if (ShouldCoerceId && Integral(raw.GetType()) && Integral(type))
            return System.Convert.ChangeType(raw, type);
        if (type == typeof(string) && raw is System.Guid fromGuid)
            return fromGuid.ToString();
        if (type == typeof(System.Guid) && raw is string guidText && System.Guid.TryParse(guidText, out var parsed))
            return parsed;
        if (type == typeof(System.DateTime) && raw is string dateText
            && System.DateTime.TryParse(
                dateText,
                CultureInfo.InvariantCulture,
                DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal,
                out var date))
            return System.DateTime.SpecifyKind(date, DateTimeKind.Utc);
        if (type == typeof(decimal) && raw is IConvertible)
            return System.Convert.ToDecimal(raw, CultureInfo.InvariantCulture);
        return null;
    }

    static bool Integral(Type type) => !type.IsEnum && Type.GetTypeCode(type) is
        TypeCode.Byte or TypeCode.SByte or TypeCode.Int16 or TypeCode.UInt16
        or TypeCode.Int32 or TypeCode.UInt32 or TypeCode.Int64 or TypeCode.UInt64;

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

public static class MetaDataRowTypeConverter
{
    public static bool Has(object? obj, string name) =>
        obj switch
        {
            null => false,
            IDictionary<string, Value> row => row.ContainsKey(name),
            IDictionary dict => dict.Contains(name),
            _ => obj.GetType().GetProperty(name) is not null
        };

    public static object? Get(object? obj, string name) =>
        obj switch
        {
            null => null,
            IDictionary<string, Value> row => row.TryGetValue(name, out var cell) && cell is not Value.Null
                ? cell.ToObject()
                : null,
            IDictionary dict => dict.Contains(name) ? dict[name] : null,
            _ => obj.GetType().GetProperty(name)?.GetValue(obj)
        };

    public static bool TrySet(object obj, string name, object? value, string? id = null)
    {
        if (obj is IDictionary<string, Value> row)
        {
            row[name] = Value.From(value);
            return true;
        }
        if (obj is IDictionary dict)
        {
            dict[name] = value;
            return true;
        }
        var prop = obj.GetType().GetProperty(name);
        if (prop is null || !prop.CanWrite) return false;
        var target = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
        if (value is IDictionary incoming && typeof(IDictionary).IsAssignableFrom(target))
        {
            var map = (IDictionary)(Activator.CreateInstance(target)
                ?? throw new DatasourceException($"Cannot create \"{name}\""));
            var valueType = target.IsGenericType && target.GetGenericArguments().Length == 2
                ? target.GetGenericArguments()[1]
                : typeof(object);
            foreach (DictionaryEntry entry in incoming)
            {
                if (entry.Key is null) continue;
                map[entry.Key] = Coerce(entry.Value, valueType);
            }
            prop.SetValue(obj, map);
            return true;
        }
        if (value is not null && IsReference(target) && !target.IsInstanceOfType(value))
        {
            var current = prop.GetValue(obj) ?? Activator.CreateInstance(target)
                ?? throw new DatasourceException($"Cannot create \"{name}\"");
            var key = IdentityProperty(target, id)
                ?? throw new DatasourceException($"\"{name}\" requires is_id");
            var idType = Nullable.GetUnderlyingType(key.PropertyType) ?? key.PropertyType;
            key.SetValue(current, idType.IsInstanceOfType(value) ? value : System.Convert.ChangeType(value, idType));
            prop.SetValue(obj, current);
            return true;
        }
        prop.SetValue(obj, value is null || target.IsInstanceOfType(value)
            ? value
            : System.Convert.ChangeType(value, target));
        return true;
    }

    public static IEnumerable<string> Names(object? value) =>
        value switch
        {
            null => [],
            IDictionary<string, Value> row => row.Keys,
            IDictionary dict => dict.Keys.Cast<object>().Select(key => key?.ToString() ?? ""),
            _ => value.GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
                .Select(prop => prop.Name)
        };

    public static void Copy(object source, object dest)
    {
        foreach (var prop in dest.GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(item => item.CanRead && item.GetIndexParameters().Length == 0))
        {
            if (Get(source, prop.Name) is { } value)
                TrySet(dest, prop.Name, value);
        }
    }

    public static object? Key(object? value, string name)
    {
        if (value is null or string || value is Value || value.GetType().IsPrimitive)
            return value;
        return Get(value, name) ?? value;
    }

    static PropertyInfo? IdentityProperty(Type target, string? id) =>
        !string.IsNullOrEmpty(id) && target.GetProperty(id) is { } named
            ? named
            : target.GetProperty("id") ?? target.GetProperty("Id");

    static object? Coerce(object? value, Type valueType)
    {
        if (value is null || valueType == typeof(object) || valueType.IsInstanceOfType(value))
            return value;
        if (valueType == typeof(string) || valueType.IsValueType)
            return Get(value, "Value") ?? Get(value, "value");
        var copy = Activator.CreateInstance(valueType);
        if (copy is null) return value;
        Copy(value, copy);
        return copy;
    }

    static bool IsReference(Type type) =>
        type.IsClass && type != typeof(string) && !typeof(IEnumerable).IsAssignableFrom(type);
}
