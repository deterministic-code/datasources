using System.Globalization;

namespace Datasources;

public sealed class FlatToTypeMapper : IFlatToTypeMapper
{
    readonly record struct GroupPlan(string Key, IReadOnlyDictionary<string, string> Fields, string DestKey);

    readonly Dictionary<string, string> scalars = [];
    readonly Dictionary<string, GroupPlan> groups = [];
    readonly string parentDest;

    public TypeMap Spec { get; }

    public FlatToTypeMapper(TypeMap spec)
    {
        Spec = spec;
        foreach (var (name, field) in spec.Fields)
            (field switch
            {
                TypeMapField.Column(var source) => (Action)(() => scalars[name] = source),
                TypeMapField.Group(var map) => () => groups[name] = Plan(map),
                _ => () => throw new InvalidOperationException($"{spec.Key}: empty field")
            })();
        parentDest = DestOf(scalars, spec.Key);
    }

    public Dictionary<string, MappedValue> Map(Row value)
    {
        var dest = new Dictionary<string, MappedValue>();
        foreach (var (name, column) in scalars)
            dest[name] = new MappedValue.Cell(Cell(value, column));
        foreach (var (name, group) in groups)
            dest[name] = new MappedValue.Many(
                CellKey(Cell(value, group.Key)) is null
                    ? []
                    : [group.Fields.ToDictionary(
                        pair => pair.Key,
                        pair => (MappedValue)new MappedValue.Cell(Cell(value, pair.Value)))]);
        return dest;
    }

    public Row ReverseMap(Dictionary<string, MappedValue> value)
    {
        var row = new Row();
        foreach (var (name, column) in scalars)
            row[column] = value.TryGetValue(name, out var mapped) && mapped is MappedValue.Cell(var cell)
                ? cell
                : Value.Null.Instance;
        foreach (var (name, group) in groups)
        {
            var child = value.TryGetValue(name, out var mapped) && mapped is MappedValue.Many(var items)
                ? items.ElementAtOrDefault(0)
                : null;
            foreach (var (field, column) in group.Fields)
                row[column] = child is not null
                    && child.TryGetValue(field, out var item)
                    && item is MappedValue.Cell(var itemCell)
                    ? itemCell
                    : Value.Null.Instance;
        }
        return row;
    }

    public IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Row> rows) =>
        Collapse(rows.Select(row => groups.Count == 0 ? Passthrough(row) : Map(row)));

    public IReadOnlyList<Dictionary<string, MappedValue>> Collapse(
        IEnumerable<Dictionary<string, MappedValue>> rows)
    {
        var byParent = new Dictionary<string, Dictionary<string, MappedValue>>();
        var seen = new HashSet<string>();
        foreach (var item in rows)
        {
            var parent = MappedKey(item, parentDest);
            if (parent is null) continue;
            if (byParent.TryAdd(parent, item))
            {
                Mark(seen, parent, item);
                continue;
            }
            Merge(byParent[parent], item, parent, seen);
        }
        return [.. byParent.Values];
    }

    void Mark(HashSet<string> seen, string parent, Dictionary<string, MappedValue> item)
    {
        foreach (var (name, group) in groups)
        {
            if (item.GetValueOrDefault(name) is not MappedValue.Many(var children)) continue;
            foreach (var child in children)
            {
                var key = MappedKey(child, group.DestKey);
                if (key is not null) seen.Add($"{parent}:{name}:{key}");
            }
        }
    }

    void Merge(
        Dictionary<string, MappedValue> into,
        Dictionary<string, MappedValue> from,
        string parent,
        HashSet<string> seen)
    {
        foreach (var (name, group) in groups)
        {
            if (into.GetValueOrDefault(name) is not MappedValue.Many(var dest)
                || from.GetValueOrDefault(name) is not MappedValue.Many(var source))
                continue;
            foreach (var child in source)
            {
                var key = MappedKey(child, group.DestKey);
                if (key is null || !seen.Add($"{parent}:{name}:{key}")) continue;
                dest.Add(child);
            }
        }
    }

    static GroupPlan Plan(TypeMapGroup map)
    {
        var fields = map.Fields
            .Where(pair => pair.Value is TypeMapField.Column)
            .ToDictionary(pair => pair.Key, pair => ((TypeMapField.Column)pair.Value).Source);
        return new(map.Key, fields, DestOf(fields, map.Key));
    }

    static string DestOf(IReadOnlyDictionary<string, string> fields, string column) =>
        fields.FirstOrDefault(pair => pair.Value == column).Key ?? column;

    static Value Cell(Row row, string column) =>
        row.TryGetValue(column, out var value) ? value : Value.Null.Instance;

    static Dictionary<string, MappedValue> Passthrough(Row row) =>
        row.ToDictionary(pair => pair.Key, pair => (MappedValue)new MappedValue.Cell(pair.Value));

    static string? MappedKey(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Cell(var value)
            ? CellKey(value)
            : null;

    static string? CellKey(Value value) => value switch
    {
        Value.Null => null,
        Value.Int(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Text(var text) => text,
        Value.Bool(var flag) => flag.ToString(),
        Value.Float(var number) => number.ToString(CultureInfo.InvariantCulture),
        _ => value.ToObject()?.ToString()
    };
}
