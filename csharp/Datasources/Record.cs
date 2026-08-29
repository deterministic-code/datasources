namespace Datasources;

public sealed class Record : Dictionary<string, Value>
{
    public Record() { }
    public Record(IEnumerable<KeyValuePair<string, Value>> pairs) : base(
        pairs as IDictionary<string, Value> ?? pairs.ToDictionary(pair => pair.Key, pair => pair.Value)) { }
}
