using Datasources;

namespace Datasources.Memory;

public sealed class MemoryStore
{
    public Dictionary<string, List<Row>> Collections { get; } = new();

    public List<Row> GetCollection(string name) =>
        Collections.TryGetValue(name, out var rows) ? rows : Collections[name] = [];

    public bool HasCollection(string name) => Collections.ContainsKey(name);

    public Dictionary<string, List<Row>> Snapshot() =>
        Collections.ToDictionary(pair => pair.Key, pair => CloneRows(pair.Value));

    public void Restore(Dictionary<string, List<Row>> snapshot)
    {
        Collections.Clear();
        foreach (var pair in snapshot) Collections[pair.Key] = CloneRows(pair.Value);
    }

    public void Seed(object data, string collection)
    {
        if (data is IDictionary<string, List<Row>> namedRows)
        {
            foreach (var pair in namedRows.Where(p => !HasCollection(p.Key)))
                Collections[pair.Key] = CloneRows(pair.Value);
            return;
        }

        if (data is IDictionary<string, List<Dictionary<string, Value>>> named)
        {
            foreach (var pair in named.Where(p => !HasCollection(p.Key)))
                Collections[pair.Key] = pair.Value.Select(AsRow).ToList();
            return;
        }

        if (data is IEnumerable<IReadOnlyDictionary<string, Value>> rows && !HasCollection(collection))
            Collections[collection] = rows.Select(AsRow).ToList();
    }

    public static List<Row> CloneRows(IEnumerable<Row> rows) =>
        rows.Select(AsRow).ToList();

    static Row AsRow(IReadOnlyDictionary<string, Value> row) => row as Row ?? new Row(row);
}
