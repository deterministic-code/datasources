using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Json;

public sealed class JsonCommand : CommandMiddlewareHost, ICommand
{
    public JsonCommand(JsonConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var filter = Merge(ParseFilter(query), Helpers.ParametersToRecord(paramsList));
            var nodes = Typed.GetCollection().Where(row => Matches(row, filter)).ToList();
            IReadOnlyList<IReadOnlyDictionary<string, object?>> rows = nodes.Select(ToRow).ToList();
            return Task.FromResult(((IReader)new ArrayReader(nodes.Cast<object?>().ToList()), rows));
        });

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var op = query.Trim().ToLowerInvariant();
            var obj = Helpers.ParametersToRecord(paramsList).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                Typed.GetCollection().Add(ToObject(obj));
                await Typed.Persist();
                return (new Response(obj, 1), EmptyRows());
            }

            if (op == "update")
            {
                var whereText = obj.TryGetValue("$where", out var where) ? where as string : null;
                obj.Remove("$where");
                var filter = whereText is not null ? ParseFilter(whereText) : Helpers.IdentityFilter(obj);
                return (await ApplyUpdate(obj, filter), EmptyRows());
            }

            if (op == "delete")
            {
                var filter = obj.Count == 0 ? new Dictionary<string, object?>() : Helpers.IdentityFilter(obj);
                return (await ApplyDelete(filter), EmptyRows());
            }

            throw new DatasourceException($"Unsupported JSON command: {query}");
        });

    private async Task<Response> ApplyUpdate(IReadOnlyDictionary<string, object?> obj, IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var affected = 0;
        for (var i = 0; i < collection.Count; i++)
        {
            if (Matches(collection[i], filter) && collection[i] is JsonObject current)
            {
                foreach (var pair in obj)
                    current[pair.Key] = JsonValue.Create(pair.Value);
                affected++;
            }
        }

        await Typed.Persist();
        return new Response(obj, affected);
    }

    private async Task<Response> ApplyDelete(IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var remaining = collection.Where(row => !Matches(row, filter)).Select(n => n?.DeepClone()).ToList();
        var affected = collection.Count - remaining.Count;
        collection.Clear();
        foreach (var row in remaining) collection.Add(row);
        await Typed.Persist();
        return new Response(null, affected);
    }

    public static Dictionary<string, object?> ParseFilter(string? text) =>
        string.IsNullOrWhiteSpace(text)
            ? new Dictionary<string, object?>()
            : JsonNode.Parse(text) is JsonObject obj
                ? obj.ToDictionary(k => k.Key, v => JsonValueToObject(v.Value))
                : throw new DatasourceException("JSON filter must be an object");

    public static bool Matches(JsonNode? row, IReadOnlyDictionary<string, object?> filter) =>
        row is JsonObject obj && filter.All(pair => Equals(JsonValueToObject(obj[pair.Key]), pair.Value));

    public static JsonObject ToObject(IReadOnlyDictionary<string, object?> obj)
    {
        var json = new JsonObject();
        foreach (var pair in obj) json[pair.Key] = JsonValue.Create(pair.Value);
        return json;
    }

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

    private static Dictionary<string, object?> ToRow(JsonNode? row) =>
        row is JsonObject obj
            ? obj.ToDictionary(k => k.Key, v => JsonValueToObject(v.Value))
            : new Dictionary<string, object?>();

    private static IReadOnlyList<IReadOnlyDictionary<string, object?>> EmptyRows() => [];
}
