using Datasources;

namespace Datasources.Memory;

public sealed class MemoryStore
{
    public Dictionary<string, List<Dictionary<string, object?>>> Collections { get; } = new();

    public List<Dictionary<string, object?>> GetCollection(string name) =>
        Collections.TryGetValue(name, out var rows) ? rows : Collections[name] = [];

    public bool HasCollection(string name) => Collections.ContainsKey(name);

    public Dictionary<string, List<Dictionary<string, object?>>> Snapshot() =>
        Collections.ToDictionary(pair => pair.Key, pair => CloneRows(pair.Value));

    public void Restore(Dictionary<string, List<Dictionary<string, object?>>> snapshot)
    {
        Collections.Clear();
        foreach (var pair in snapshot) Collections[pair.Key] = CloneRows(pair.Value);
    }

    public void Seed(object data, string collection)
    {
        if (data is IDictionary<string, List<Dictionary<string, object?>>> named)
        {
            foreach (var pair in named.Where(p => !HasCollection(p.Key)))
                Collections[pair.Key] = CloneRows(pair.Value);
            return;
        }

        if (data is IEnumerable<Dictionary<string, object?>> rows && !HasCollection(collection))
            Collections[collection] = CloneRows(rows);
    }

    public static List<Dictionary<string, object?>> CloneRows(IEnumerable<Dictionary<string, object?>> rows) =>
        rows.Select(row => new Dictionary<string, object?>(row)).ToList();
}
