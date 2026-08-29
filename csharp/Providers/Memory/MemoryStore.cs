using Datasources;

namespace Datasources.Memory;

public sealed class MemoryStore
{
    public Dictionary<string, List<Dictionary<string, Value>>> Collections { get; } = new();

    public List<Dictionary<string, Value>> GetCollection(string name) =>
        Collections.TryGetValue(name, out var rows) ? rows : Collections[name] = [];

    public bool HasCollection(string name) => Collections.ContainsKey(name);

    public Dictionary<string, List<Dictionary<string, Value>>> Snapshot() =>
        Collections.ToDictionary(pair => pair.Key, pair => CloneRows(pair.Value));

    public void Restore(Dictionary<string, List<Dictionary<string, Value>>> snapshot)
    {
        Collections.Clear();
        foreach (var pair in snapshot) Collections[pair.Key] = CloneRows(pair.Value);
    }

    public void Seed(object data, string collection)
    {
        if (data is IDictionary<string, List<Dictionary<string, Value>>> named)
        {
            foreach (var pair in named.Where(p => !HasCollection(p.Key)))
                Collections[pair.Key] = CloneRows(pair.Value);
            return;
        }

        if (data is IEnumerable<Dictionary<string, Value>> rows && !HasCollection(collection))
            Collections[collection] = CloneRows(rows);
    }

    public static List<Dictionary<string, Value>> CloneRows(IEnumerable<Dictionary<string, Value>> rows) =>
        rows.Select(row => new Dictionary<string, Value>(row)).ToList();
}
