using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Memory;

internal sealed class MemoryCommand : CommandMiddlewareHost, ICommand
{
    public MemoryCommand(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(text.Text, parameters);

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Execute(text.Text, parameters);

    public override Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var filter = Merge(ParseFilter(query), Helpers.ParametersToRecord(paramsList));
            IReadOnlyList<Row> rows = [.. Typed.GetCollection().Where(row => Matches(row, filter))];
            return Task.FromResult(((IReader<Row>)new ArrayReader<Row>(rows), rows));
        });

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement switch
        {
            SelectInsertedIdStatement inserted => Run($"{inserted.Table}.{inserted.Column}", parameters, (_, _) =>
            {
                Typed.EnsureOpen();
                var id = Typed.Inserted(inserted.Table);
                IReadOnlyList<Row> rows = id is Value.Null ? [] : [new Row { [inserted.Column] = id }];
                return Task.FromResult(((IReader<Row>)new ArrayReader<Row>(rows), rows));
            }),
            ISelectStatement { Where: Where clause } => Run(statement.GetText(), parameters, (_, _) =>
            {
                Typed.EnsureOpen();
                var evaluator = new StatementEvaluator();
                IReadOnlyList<Row> rows = [.. Rows(TableOf(statement), create: false).Where(row =>
                    evaluator.Evaluate(clause, row) is Value.Bool(true))];
                return Task.FromResult(((IReader<Row>)new ArrayReader<Row>(rows), rows));
            }),
            ISelectStatement query when !LooksJson(query.Where?.GetText()) && !LooksJson(statement.GetText()) =>
                Run(statement.GetText(), parameters, (_, _) =>
                {
                    Typed.EnsureOpen();
                    IReadOnlyList<Row> rows = [.. Rows(TableOf(statement), create: false)];
                    return Task.FromResult(((IReader<Row>)new ArrayReader<Row>(rows), rows));
                }),
            _ => ExecuteReader(FilterText(statement), parameters)
        };

    public Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement switch
        {
            IInsertStatement => Write("insert", parameters, TableOf(statement), create: true),
            IUpdateStatement update => Write("update", parameters, TableOf(update), create: false, update.Where),
            IDeleteStatement delete => Write("delete", parameters, TableOf(delete), create: false, delete.Where),
            _ => Execute(statement.GetText(), parameters)
        };

    public override Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var op = query.Trim().ToLowerInvariant();
            var obj = Helpers.ParametersToRecord(paramsList).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                var id = InsertedId(obj, Typed.GetCollection());
                obj["id"] = id;
                Typed.GetCollection().Add(new Row(obj));
                Typed.RememberInserted(Typed.Collection, id);
                return Task.FromResult((new Response(id, 1, paramsList), EmptyRows()));
            }

            if (op == "update")
            {
                var whereText = obj.TryGetValue("$where", out var where) && where is Value.Text text ? text.Item : null;
                obj.Remove("$where");
                var filter = whereText is not null ? ParseFilter(whereText) : new Row(obj);
                var updated = ApplyUpdate(Typed.GetCollection(), new Row(obj), filter);
                return Task.FromResult((new Response(updated.Result, updated.RowsAffected, paramsList), EmptyRows()));
            }

            if (op == "delete")
            {
                var deleted = ApplyDelete(Typed.GetCollection(), new Row(obj));
                return Task.FromResult((new Response(deleted.Result, deleted.RowsAffected, paramsList), EmptyRows()));
            }

            throw new DatasourceException($"Unsupported Memory command: {query}");
        });

    private Response ApplyUpdate(List<Row> collection, Row obj, Row filter)
    {
        var affected = 0;
        foreach (var row in collection.Where(row => Matches(row, filter)))
        {
            foreach (var pair in obj) row[pair.Key] = pair.Value;
            affected++;
        }

        return new Response(Value.Null.Instance, affected);
    }

    private Response ApplyDelete(List<Row> collection, Row filter)
    {
        var affected = collection.RemoveAll(row => Matches(row, filter));
        return new Response(Value.Null.Instance, affected);
    }

    Task<Response> Write(string op, IReadOnlyList<Parameter>? parameters, string? table, bool create, IWhere? where = null) =>
        Run(op, parameters, (_, paramsList) =>
        {
            Typed.EnsureOpen();
            var rows = Rows(table, create);
            var obj = Helpers.ParametersToRecord(paramsList).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                var id = InsertedId(obj, rows);
                obj["id"] = id;
                rows.Add(new Row(obj));
                Typed.RememberInserted(table ?? Typed.Collection, id);
                return Task.FromResult((new Response(id, 1, paramsList), EmptyRows()));
            }

            if (op == "update")
                return Task.FromResult((ApplyChanged(rows, obj, where, paramsList), EmptyRows()));

            return Task.FromResult((RemoveMatched(rows, obj, where, paramsList), EmptyRows()));
        });

    Response ApplyChanged(List<Row> rows, Dictionary<string, Value> obj, IWhere? where, IReadOnlyList<Parameter>? parameters)
    {
        if (where is Where clause)
        {
            var evaluator = new StatementEvaluator();
            var names = SetNames(parameters, where);
            var affected = 0;
            foreach (var row in rows.Where(row => evaluator.Evaluate(clause, row) is Value.Bool(true)))
            {
                foreach (var name in names)
                    if (obj.TryGetValue(name, out var value)) row[name] = value;
                affected++;
            }

            return new Response(Value.Null.Instance, affected, parameters);
        }

        obj.Remove("$where");
        var filter = LooksJson(where?.GetText()) ? ParseFilter(where!.GetText()) : new Row(obj);
        var updated = ApplyUpdate(rows, new Row(obj), filter);
        return new Response(updated.Result, updated.RowsAffected, parameters);
    }

    Response RemoveMatched(List<Row> rows, Dictionary<string, Value> obj, IWhere? where, IReadOnlyList<Parameter>? parameters)
    {
        if (where is Where clause)
        {
            var evaluator = new StatementEvaluator();
            var affected = rows.RemoveAll(row => evaluator.Evaluate(clause, row) is Value.Bool(true));
            return new Response(Value.Null.Instance, affected, parameters);
        }

        var filter = LooksJson(where?.GetText()) ? ParseFilter(where!.GetText()) : new Row(obj);
        var deleted = ApplyDelete(rows, filter);
        return new Response(deleted.Result, deleted.RowsAffected, parameters);
    }

    List<Row> Rows(string? table, bool create) =>
        table is { Length: > 0 } name && (create || Typed.Store.HasCollection(name))
            ? Typed.Store.GetCollection(name)
            : Typed.GetCollection();

    static string? TableOf(IStatement statement) =>
        statement is SelectStatement { Table: { Length: > 0 } table } ? table : TableIn(statement.GetText());

    static string? TableIn(string text)
    {
        var sql = text.TrimStart();
        var skip = sql.StartsWith("INSERT INTO ", StringComparison.OrdinalIgnoreCase) ? "INSERT INTO ".Length
            : sql.StartsWith("DELETE FROM ", StringComparison.OrdinalIgnoreCase) ? "DELETE FROM ".Length
            : sql.StartsWith("UPDATE ", StringComparison.OrdinalIgnoreCase) ? "UPDATE ".Length
            : sql.IndexOf(" FROM ", StringComparison.OrdinalIgnoreCase) is var from and >= 0 ? from + " FROM ".Length
            : -1;
        if (skip < 0 || skip >= sql.Length) return null;
        var end = skip;
        while (end < sql.Length && !char.IsWhiteSpace(sql[end]) && sql[end] != '(') end++;
        var name = sql[skip..end];
        return name.Length == 0 ? null : name;
    }

    static IReadOnlyList<string> SetNames(IReadOnlyList<Parameter>? parameters, IWhere where)
    {
        var bound = where.Parameters.Select(parameter => parameter.Name).ToHashSet();
        return [.. (parameters ?? []).Select(parameter => parameter.Name).Where(name => !bound.Contains(name) && !name.EndsWith("__where", StringComparison.Ordinal))];
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

    static string FilterText(IStatement statement)
    {
        var where = statement is ISelectStatement query ? query.Where?.GetText() : null;
        if (LooksJson(where)) return where!;
        var text = statement.GetText();
        return LooksJson(text) || string.IsNullOrWhiteSpace(text) ? text : "";
    }

    static bool LooksJson(string? text) => text?.TrimStart().StartsWith('{') == true;

    static Value InsertedId(Dictionary<string, Value> row, List<Row> rows) =>
        row.TryGetValue("id", out var existing) && existing is Value.Int
            ? existing
            : rows.Select(item => item.TryGetValue("id", out var cell) && cell is Value.Int id ? id.Item : 0L).DefaultIfEmpty(0).Max() + 1;

    private static IReadOnlyList<Row> EmptyRows() => [];
}
