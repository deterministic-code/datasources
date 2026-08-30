using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Memory;

public sealed class MemoryCommand : CommandMiddlewareHost, ICommand
{
    public MemoryCommand(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task<IReader> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var filter = Merge(ParseFilter(query), Helpers.ParametersToRecord(paramsList));
            IReadOnlyList<Row> rows = [.. Typed.GetCollection().Where(row => Matches(row, filter))];
            return Task.FromResult(((IReader)new ArrayReader(rows.Cast<object?>().ToList()), rows));
        });

    public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var op = query.Trim().ToLowerInvariant();
            var obj = Helpers.ParametersToRecord(paramsList).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                Typed.GetCollection().Add(new Row(obj));
                return Task.FromResult((new Response(Value.Null.Instance, 1), EmptyRows()));
            }

            if (op == "update")
            {
                var whereText = obj.TryGetValue("$where", out var where) && where is Value.Text text ? text.Item : null;
                obj.Remove("$where");
                var filter = whereText is not null ? ParseFilter(whereText) : new Row(Helpers.IdentityFilter(obj));
                return Task.FromResult((ApplyUpdate(new Row(obj), filter), EmptyRows()));
            }

            if (op == "delete")
                return Task.FromResult((ApplyDelete(obj.Count == 0 ? new Row() : new Row(Helpers.IdentityFilter(obj))), EmptyRows()));

            throw new DatasourceException($"Unsupported Memory command: {query}");
        });

    private Response ApplyUpdate(Row obj, Row filter)
    {
        var collection = Typed.GetCollection();
        var affected = 0;
        foreach (var row in collection.Where(row => Matches(row, filter)))
        {
            foreach (var pair in obj) row[pair.Key] = pair.Value;
            affected++;
        }

        return new Response(Value.Null.Instance, affected);
    }

    private Response ApplyDelete(Row filter)
    {
        var collection = Typed.GetCollection();
        var affected = collection.RemoveAll(row => Matches(row, filter));
        return new Response(Value.Null.Instance, affected);
    }

    public static Row ParseFilter(string? text) =>
        string.IsNullOrWhiteSpace(text)
            ? new Row()
            : JsonNode.Parse(text) is JsonObject obj
                ? new Row(obj.ToDictionary(k => k.Key, v => JsonValueToValue(v.Value)))
                : throw new DatasourceException("Memory filter must be an object");

    public static bool Matches(Row row, Row filter) =>
        filter.All(pair => Equals(row.TryGetValue(pair.Key, out var value) ? value : Value.Null.Instance, pair.Value));

    private static Row Merge(Row left, Row right)
    {
        foreach (var pair in right) left[pair.Key] = pair.Value;
        return left;
    }

    public static Value JsonValueToValue(JsonNode? node)
    {
        if (node is not JsonValue value) return Value.From(node?.ToString());
        if (value.TryGetValue<int>(out var i)) return i;
        if (value.TryGetValue<long>(out var l)) return l;
        if (value.TryGetValue<bool>(out var b)) return b;
        if (value.TryGetValue<double>(out var d)) return d;
        if (value.TryGetValue<string>(out var s)) return s;
        return value.ToString();
    }

    private static IReadOnlyList<Row> EmptyRows() => [];
}
