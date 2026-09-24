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
        var item = new RowTypeConverter<TItem>().Convert(row);
        foreach (var (key, mapped) in value)
            if (mapped is MappedValue.Many(var items)
                && typeof(TItem).GetProperty(key) is { CanWrite: true } prop
                && Element(prop.PropertyType) is { } element)
                prop.SetValue(item, Children(element, items));
        return item;
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
}
