using YamlDotNet.Serialization;

namespace Datasources.CodeGenerators;

public sealed class YamlCursor
{
    static readonly IDeserializer Deserializer = new DeserializerBuilder().Build();

    readonly object? value;

    public YamlCursor(object? value) => this.value = value;

    public static YamlCursor FromYaml(string text) => new(Deserializer.Deserialize<object>(text));

    public object? Value => value;

    public YamlCursor Child(string key) => new(At(key));

    public bool Has(string key) => At(key) is not null || MapHas(key);

    public string? Str(string key) => At(key) switch
    {
        string text => text,
        bool flag => flag ? "true" : "false",
        _ => At(key)?.ToString(),
    };

    public bool Bool(string key) => At(key) is true or "true" or "True";

    public object? Literal(string key) => IsLiteral(At(key)) ? At(key) : null;

    public IReadOnlyList<string> Strings(string key) => At(key) switch
    {
        List<object> list => list.Select(item => Convert.ToString(item) ?? "").ToList(),
        IEnumerable<object> items => items.Select(item => Convert.ToString(item) ?? "").ToList(),
        string text => [text],
        _ => [],
    };

    public IReadOnlyList<YamlCursor> Items() =>
        Sequence().Select(item => new YamlCursor(item)).ToList();

    public IReadOnlyList<(string Name, YamlCursor Node)> NamedItems()
    {
        var items = new List<(string, YamlCursor)>();
        foreach (var item in Items())
        {
            var first = item.Pairs().FirstOrDefault();
            if (first.Key is not string name) continue;
            items.Add((name, new YamlCursor(first.Value)));
        }
        return items;
    }

    public IReadOnlyList<(string Name, YamlCursor Node)> NamedList(string key) => Child(key).NamedItems();

    object? At(string key)
    {
        foreach (var (k, v) in Pairs())
            if (k is string name && name == key) return v;
        return null;
    }

    bool MapHas(string key) => Pairs().Any(pair => pair.Key is string name && name == key);

    IEnumerable<KeyValuePair<object, object>> Pairs() => value switch
    {
        Dictionary<object, object> map => map,
        Dictionary<string, object> map => map.Select(pair => new KeyValuePair<object, object>(pair.Key, pair.Value)),
        IDictionary<string, object> map => map.Select(pair => new KeyValuePair<object, object>(pair.Key, pair.Value)),
        _ => [],
    };

    IEnumerable<object> Sequence() => value switch
    {
        List<object> list => list,
        IEnumerable<object> items when value is not string => items,
        _ => [],
    };

    static bool IsLiteral(object? v) => v is null or string or bool or byte or sbyte or short or ushort or int or uint or long or ulong or float or double or decimal;
}
