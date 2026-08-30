namespace Datasources;

public sealed class Row : Dictionary<string, Value>
{
    public Row() { }
    public Row(IEnumerable<KeyValuePair<string, Value>> pairs) : base(
        pairs as IDictionary<string, Value> ?? pairs.ToDictionary(pair => pair.Key, pair => pair.Value)) { }
}
