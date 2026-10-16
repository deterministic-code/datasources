using YamlDotNet.Serialization;

namespace Datasources.CodeGenerators;

public static class VersionDelta
{
    static readonly IDeserializer Deserializer = new DeserializerBuilder().Build();
    static readonly ISerializer Serializer = new SerializerBuilder().Build();

    public static object? Parse(string yaml) => Deserializer.Deserialize<object>(yaml);

    public static string Yaml(object? value) => Serializer.Serialize(value);

    public static Dictionary<string, object?> ApplyDocument(
        IReadOnlyDictionary<string, object?> before,
        IReadOnlyDictionary<string, object?> doc)
    {
        var output = new Dictionary<string, object?>(before, StringComparer.Ordinal);
        foreach (var (path, envelope) in doc)
        {
            var next = ApplyFile(output.GetValueOrDefault(path), envelope);
            if (next is null) output.Remove(path);
            else output[path] = next;
        }
        return output;
    }

    public static Dictionary<string, object?> AsDocument(object? value, string source) =>
        AsMap(value) ?? throw new InvalidOperationException($"{source}: expected a mapping");

    public static bool Equal(object? left, object? right) => Yaml(left) == Yaml(right);

    static object? ApplyFile(object? before, object? envelope)
    {
        if (envelope is null) return before;
        if (AsMap(envelope) is { } map)
        {
            if (IsTrue(map, "removed") && map.Count == 1) return null;
            if (IsTrue(map, "added")) return map.GetValueOrDefault("content");
        }
        return Apply(before ?? null, envelope);
    }

    static object? Apply(object? before, object? next)
    {
        if (next is null) return before;
        if (IsKeyedCollection(before) && IsKeyedSplit(next)) return ApplyKeyed(AsList(before)!, AsMap(next)!);
        if (AsMap(before) is { } beforeMap && AsMap(next) is { } nextMap) return ApplyObject(beforeMap, nextMap);
        return next;
    }

    static List<object?> ApplyKeyed(List<object?> beforeVal, Dictionary<string, object?> split)
    {
        var removed = new HashSet<string>(Strings(split, "removed"), StringComparer.Ordinal);
        var changed = new Dictionary<string, object?>(StringComparer.Ordinal);
        foreach (var entry in AsList(split.GetValueOrDefault("changed")) ?? [])
        {
            var (key, value) = FirstPair(entry);
            if (key is not null) changed[key] = value;
        }
        var output = new List<object?>();
        foreach (var entry in beforeVal)
        {
            var (key, value) = FirstPair(entry);
            if (key is null || removed.Contains(key)) continue;
            output.Add(changed.TryGetValue(key, out var nested) ? MapOf(key, Apply(value, nested)) : entry);
        }
        output.AddRange(AsList(split.GetValueOrDefault("added")) ?? []);
        return output;
    }

    static Dictionary<string, object?> ApplyObject(Dictionary<string, object?> beforeVal, Dictionary<string, object?> next)
    {
        var removed = new HashSet<string>(Strings(next, "removed"), StringComparer.Ordinal);
        var output = new Dictionary<string, object?>(StringComparer.Ordinal);
        foreach (var (key, value) in beforeVal)
        {
            if (removed.Contains(key)) continue;
            output[key] = next.ContainsKey(key) ? Apply(value, next[key]) : value;
        }
        foreach (var (key, value) in next)
        {
            if (key == "removed" && next["removed"] is IEnumerable<object>) continue;
            if (!beforeVal.ContainsKey(key)) output[key] = value;
        }
        return output;
    }

    static bool IsKeyedSplit(object? value)
    {
        if (AsMap(value) is not { } map) return false;
        var allowed = new HashSet<string>(["added", "changed", "removed"], StringComparer.Ordinal);
        return map.Keys.All(allowed.Contains);
    }

    static bool IsKeyedCollection(object? value) =>
        AsList(value) is { Count: > 0 } list && list.All(item => AsMap(item) is { Count: 1 });

    static Dictionary<string, object?>? AsMap(object? value) => value switch
    {
        Dictionary<string, object?> map => map,
        Dictionary<object, object> map => map.ToDictionary(pair => Convert.ToString(pair.Key) ?? "", pair => (object?)pair.Value, StringComparer.Ordinal),
        _ => null
    };

    static List<object?>? AsList(object? value) => value switch
    {
        List<object?> list => list,
        IEnumerable<object> items when value is not string => items.Select(item => (object?)item).ToList(),
        _ => null
    };

    static IReadOnlyList<string> Strings(Dictionary<string, object?> map, string key) =>
        AsList(map.GetValueOrDefault(key))?.Select(item => Convert.ToString(item) ?? "").ToList() ?? [];

    static (string? Key, object? Value) FirstPair(object? entry)
    {
        if (AsMap(entry) is not { } map) return default;
        var pair = map.FirstOrDefault();
        return (pair.Key, pair.Value);
    }

    static Dictionary<string, object?> MapOf(string key, object? value) => new(StringComparer.Ordinal) { [key] = value };

    static bool IsTrue(Dictionary<string, object?> map, string key) =>
        map.GetValueOrDefault(key) is true or "true" or "True";
}
