using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Memory;

public sealed class MemoryCommand : MiddlewareHost, ICommand
{
    public MemoryCommand(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("executeReader", () =>
        {
            Typed.EnsureOpen();
            var filter = Merge(ParseFilter(text), Helpers.ParametersToRecord(parameters));
            var rows = Typed.GetCollection().Where(row => Matches(row, filter)).Cast<object?>().ToList();
            return Task.FromResult<IReader>(new ArrayReader(rows));
        }, text, parameters);

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("execute", () =>
        {
            Typed.EnsureOpen();
            var op = (text ?? "").Trim().ToLowerInvariant();
            var obj = Helpers.ParametersToRecord(parameters).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                Typed.GetCollection().Add(new Dictionary<string, object?>(obj));
                return Task.FromResult(new Response(obj, 1));
            }

            if (op == "update")
            {
                var whereText = obj.TryGetValue("$where", out var where) ? where as string : null;
                obj.Remove("$where");
                var filter = whereText is not null ? ParseFilter(whereText) : Helpers.IdentityFilter(obj);
                return Task.FromResult(ApplyUpdate(obj, filter));
            }

            if (op == "delete")
                return Task.FromResult(ApplyDelete(obj.Count == 0 ? new Dictionary<string, object?>() : Helpers.IdentityFilter(obj)));

            throw new DatasourceException($"Unsupported Memory command: {text ?? ""}");
        }, text, parameters);

    private Response ApplyUpdate(IReadOnlyDictionary<string, object?> obj, IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var affected = 0;
        foreach (var row in collection.Where(row => Matches(row, filter)))
        {
            foreach (var pair in obj) row[pair.Key] = pair.Value;
            affected++;
        }

        return new Response(obj, affected);
    }

    private Response ApplyDelete(IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var affected = collection.RemoveAll(row => Matches(row, filter));
        return new Response(null, affected);
    }

    public static Dictionary<string, object?> ParseFilter(string? text) =>
        string.IsNullOrWhiteSpace(text)
            ? new Dictionary<string, object?>()
            : JsonNode.Parse(text) is JsonObject obj
                ? obj.ToDictionary(k => k.Key, v => JsonValueToObject(v.Value))
                : throw new DatasourceException("Memory filter must be an object");

    public static bool Matches(IReadOnlyDictionary<string, object?> row, IReadOnlyDictionary<string, object?> filter) =>
        filter.All(pair => Equals(row.TryGetValue(pair.Key, out var value) ? value : null, pair.Value));

    private static Dictionary<string, object?> Merge(Dictionary<string, object?> left, IReadOnlyDictionary<string, object?> right)
    {
        foreach (var pair in right) left[pair.Key] = pair.Value;
        return left;
    }

    public static object? JsonValueToObject(JsonNode? node)
    {
        if (node is not JsonValue value) return node;
        if (value.TryGetValue<int>(out var i)) return i;
        if (value.TryGetValue<long>(out var l)) return l;
        if (value.TryGetValue<bool>(out var b)) return b;
        if (value.TryGetValue<double>(out var d)) return d;
        if (value.TryGetValue<string>(out var s)) return s;
        return value.ToString();
    }
}
