using System.Collections;
using System.Reflection;

namespace Datasources;

public sealed class MappedValueMapper<T> : IMapper<Dictionary<string, MappedValue>, T> where T : new()
{
    public T Map(Dictionary<string, MappedValue> value) => Item<T>(value);

    public Dictionary<string, MappedValue> ReverseMap(T value) =>
        throw new InvalidOperationException();

    static TItem Item<TItem>(Dictionary<string, MappedValue> value) where TItem : new()
    {
        var row = new Row();
        foreach (var (key, mapped) in value)
            if (mapped is MappedValue.Cell(var cell))
                row[key] = cell;
        var item = new MetaDataRowTypeConverter<TItem>().Convert(row);
        foreach (var (key, mapped) in value)
            if (mapped is MappedValue.Many(var items)
                && typeof(TItem).GetProperty(key) is { CanWrite: true } list
                && Element(list.PropertyType) is { } element)
                list.SetValue(item, Children(element, items));
            else if (mapped is MappedValue.Map(var entries))
                SetMap(item, key, entries);
        return item;
    }

    static void SetMap<TItem>(TItem item, string key, Dictionary<string, Value> entries)
    {
        var prop = typeof(TItem).GetProperty(key)
            ?? throw new DatasourceException($"Unknown property \"{key}\"");
        if (!prop.CanWrite || Arguments(prop.PropertyType) is not (var mapKey, var mapValue)
            || mapKey != typeof(string) || !IsColumn(mapValue))
            throw new DatasourceException($"{key} requires a string dictionary of column values");
        var map = (IDictionary)Activator.CreateInstance(prop.PropertyType)!;
        foreach (var (entry, cell) in entries)
            map[entry] = cell.ToObject() is { } raw && mapValue.IsInstanceOfType(raw)
                ? raw
                : cell is Value.Null ? null : Convert.ChangeType(cell.ToObject(), mapValue);
        prop.SetValue(item, map);
    }

    static object Children(Type element, List<Dictionary<string, MappedValue>> items) =>
        typeof(MappedValueMapper<T>)
            .GetMethod(nameof(List), BindingFlags.Static | BindingFlags.NonPublic)!
            .MakeGenericMethod(element)
            .Invoke(null, [items])!;

    static List<TChild> List<TChild>(List<Dictionary<string, MappedValue>> items) where TChild : new() =>
        items.Select(Item<TChild>).ToList();

    static Type? Element(Type type) =>
        type.IsGenericType && type.GetGenericTypeDefinition() == typeof(List<>)
            ? type.GetGenericArguments()[0]
            : null;

    static (Type Key, Type Value)? Arguments(Type type) =>
        type.IsGenericType && type.GetGenericTypeDefinition() == typeof(Dictionary<,>)
            ? (type.GetGenericArguments()[0], type.GetGenericArguments()[1])
            : null;

    static bool IsColumn(Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        return type == typeof(string) || type == typeof(byte[])
            || type.IsValueType && !typeof(IEnumerable).IsAssignableFrom(type);
    }
}
