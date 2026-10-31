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
                SetReferenceId(target, prop, Converted(key, value, reverse: false));
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
            if (ReferenceId(prop.GetValue(source)) is { } id)
                yield return new(ReferenceColumns.Map(prop.Name), Converted(prop.Name, id, reverse: true));
            yield break;
        }
        if (!IsColumn(prop.PropertyType)) yield break;
        yield return prop.GetValue(source) is { } raw
            ? new(prop.Name, Converted(prop.Name, Value.From(raw), reverse: true))
            : new(prop.Name, Value.Null.Instance);
    }

    static void SetReferenceId(T target, PropertyInfo prop, Value cell)
    {
        if (cell.ToObject() is not { } raw) return;
        var type = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
        var current = prop.GetValue(target) ?? Activator.CreateInstance(type);
        if (current is null) return;
        var id = type.GetProperty("Id") ?? type.GetProperty("id");
        if (id is null) return;
        var idType = Nullable.GetUnderlyingType(id.PropertyType) ?? id.PropertyType;
        id.SetValue(current, idType.IsInstanceOfType(raw) ? raw : System.Convert.ChangeType(raw, idType));
        prop.SetValue(target, current);
    }

    static Value? ReferenceId(object? value)
    {
        if (value is null) return null;
        var id = value.GetType().GetProperty("Id") ?? value.GetType().GetProperty("id");
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
