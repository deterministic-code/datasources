using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Json;

internal sealed class JsonCommand : CommandMiddlewareHost, ICommand
{
    public JsonCommand(JsonConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(text.Text, parameters);

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Execute(text.Text, parameters);

    public override Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var filter = Merge(ParseFilter(query), Helpers.ParametersToRecord(paramsList));
            var nodes = Typed.GetCollection().Where(row => Matches(row, filter)).ToList();
            IReadOnlyList<Row> rows = [.. nodes.Select(ToRow)];
            return Task.FromResult(((IReader<Row>)new ArrayReader<Row>(rows), rows));
        });

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement is SelectInsertedIdStatement inserted
            ? Run($"{inserted.Table}.{inserted.Column}", parameters, (_, _) =>
            {
                Typed.EnsureOpen();
                IReadOnlyList<Row> rows = [];
                return Task.FromResult(((IReader<Row>)new ArrayReader<Row>(rows), rows));
            })
            : ExecuteReader(FilterText(statement), parameters);

    public Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement switch
        {
            IInsertStatement => Execute("insert", parameters),
            IUpdateStatement update => Execute("update", WithWhere(parameters, update.Where)),
            IDeleteStatement delete => Execute("delete", WithWhere(parameters, delete.Where)),
            _ => Execute(DefaultStatementTextRenderer.Instance.Render(statement), parameters)
        };

    public override Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(text)
            ? throw new InvalidOperationException("empty command")
            : Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var op = query.Trim().ToLowerInvariant();
            var obj = Helpers.ParametersToRecord(paramsList).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                Typed.GetCollection().Add(ToObject(new Row(obj)));
                await Typed.Persist();
                return (new Response(Value.Null.Instance, 1, paramsList), EmptyRows());
            }

            if (op == "update")
            {
                var whereText = obj.TryGetValue("$where", out var where) && where is Value.Text text ? text.Item : null;
                obj.Remove("$where");
                var filter = whereText is not null ? ParseFilter(whereText) : new Row(obj);
                var updated = await ApplyUpdate(new Row(obj), filter);
                return (new Response(updated.Result, updated.RowsAffected, paramsList), EmptyRows());
            }

            if (op == "delete")
            {
                var filter = new Row(obj);
                var deleted = await ApplyDelete(filter);
                return (new Response(deleted.Result, deleted.RowsAffected, paramsList), EmptyRows());
            }

            throw new DatasourceException($"Unsupported JSON command: {query}");
        });

    private async Task<Response> ApplyUpdate(Row obj, Row filter)
    {
        var collection = Typed.GetCollection();
        var affected = 0;
        for (var i = 0; i < collection.Count; i++)
        {
            if (Matches(collection[i], filter) && collection[i] is JsonObject current)
            {
                foreach (var pair in obj)
                    current[pair.Key] = JsonValue.Create(pair.Value.ToObject());
                affected++;
            }
        }

        await Typed.Persist();
        return new Response(Value.Null.Instance, affected);
    }

    private async Task<Response> ApplyDelete(Row filter)
    {
        var collection = Typed.GetCollection();
        var remaining = collection.Where(row => !Matches(row, filter)).Select(n => n?.DeepClone()).ToList();
        var affected = collection.Count - remaining.Count;
        collection.Clear();
        foreach (var row in remaining) collection.Add(row);
        await Typed.Persist();
        return new Response(Value.Null.Instance, affected);
    }

    public static Row ParseFilter(string? text) =>
        string.IsNullOrWhiteSpace(text)
            ? new Row()
            : JsonNode.Parse(text) is JsonObject obj
                ? new Row(obj.ToDictionary(k => k.Key, v => JsonValueToValue(v.Value)))
                : throw new DatasourceException("JSON filter must be an object");

    public static bool Matches(JsonNode? row, Row filter) =>
        row is JsonObject obj && filter.All(pair => Equals(JsonValueToValue(obj[pair.Key]), pair.Value));

    public static JsonObject ToObject(Row obj)
    {
        var json = new JsonObject();
        foreach (var pair in obj) json[pair.Key] = JsonValue.Create(pair.Value.ToObject());
        return json;
    }

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

    private static Row ToRow(JsonNode? row) =>
        row is JsonObject obj
            ? new Row(obj.ToDictionary(k => k.Key, v => JsonValueToValue(v.Value)))
            : new Row();

    static string FilterText(IStatement statement)
    {
        var where = statement is ISelectStatement query ? RenderWhere(query.Where) : null;
        if (LooksJson(where)) return where!;
        var text = DefaultStatementTextRenderer.Instance.Render(statement);
        return LooksJson(text) || string.IsNullOrWhiteSpace(text) ? text : "";
    }

    static IReadOnlyList<Parameter>? WithWhere(IReadOnlyList<Parameter>? parameters, IWhere? where) =>
        LooksJson(RenderWhere(where))
            ? [.. parameters ?? [], new Parameter("$where", RenderWhere(where)!)]
            : parameters;

    static string? RenderWhere(IWhere? where) =>
        where is null ? null : DefaultStatementTextRenderer.Instance.Render(where);

    static bool LooksJson(string? text) => text?.TrimStart().StartsWith('{') == true;

    private static IReadOnlyList<Row> EmptyRows() => [];
}
