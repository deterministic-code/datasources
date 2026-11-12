using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Memory;

internal sealed class MemoryEngine
{
    readonly Dictionary<string, List<Row>> collections = new();
    readonly Dictionary<string, Value> inserted = new(StringComparer.Ordinal);
    Dictionary<string, List<Row>>? snapshot;

    public string Collection { get; set; } = "";
    public bool InTransaction { get; private set; }
    internal IReadOnlyList<Row> Records { get; private set; } = [];

    public List<Row> GetCollection(string name) =>
        collections.TryGetValue(name, out var rows) ? rows : collections[name] = [];

    public bool HasCollection(string name) => collections.ContainsKey(name);

    public void Begin()
    {
        snapshot = collections.ToDictionary(pair => pair.Key, pair => CloneRows(pair.Value));
        InTransaction = true;
    }

    public void Commit()
    {
        InTransaction = false;
        snapshot = null;
    }

    public void Abort()
    {
        if (snapshot is not null)
        {
            collections.Clear();
            foreach (var pair in snapshot) collections[pair.Key] = CloneRows(pair.Value);
        }
        InTransaction = false;
        snapshot = null;
    }

    public void Seed(object data, string collection)
    {
        if (data is IDictionary<string, List<Row>> namedRows)
        {
            foreach (var pair in namedRows.Where(p => !HasCollection(p.Key)))
                collections[pair.Key] = CloneRows(pair.Value);
            return;
        }

        if (data is IDictionary<string, List<Dictionary<string, Value>>> named)
        {
            foreach (var pair in named.Where(p => !HasCollection(p.Key)))
                collections[pair.Key] = pair.Value.Select(AsRow).ToList();
            return;
        }

        if (data is IEnumerable<IReadOnlyDictionary<string, Value>> rows && !HasCollection(collection))
            collections[collection] = rows.Select(AsRow).ToList();
    }

    public Response Execute(string text, IReadOnlyList<Parameter>? parameters = null)
    {
        Records = [];
        if (TryNamed(text, "insert", out _))
            return Named("insert", parameters, null);
        if (TryNamed(text, "update", out var updatedWhere))
            return Named("update", parameters, updatedWhere);
        if (TryNamed(text, "delete", out var deletedWhere))
            return Named("delete", parameters, deletedWhere);
        if (LooksJson(text) || string.IsNullOrWhiteSpace(text))
        {
            var filter = Merge(ParseFilter(text), Helpers.ParametersToRecord(parameters));
            if (filter.TryGetValue("__inserted", out var table) && table is Value.Text name)
            {
                var column = filter.TryGetValue("__column", out var col) && col is Value.Text textCol ? textCol.Item : "id";
                Records = InsertedRow(name.Item, column);
                return new Response(Value.Null.Instance, Records.Count, parameters);
            }
            var collection = Collection;
            if (filter.TryGetValue("__table", out var named) && named is Value.Text tableName)
            {
                collection = tableName.Item;
                filter.Remove("__table");
            }
            Records = [.. GetCollection(collection).Where(row => Matches(row, filter))];
            return new Response(Value.Null.Instance, Records.Count, parameters);
        }
        var commandText = text;
        IWhere? jsonWhere = null;
        if (TryJsonWhere(text, out var sql, out var json))
        {
            commandText = sql;
            jsonWhere = new WhereText(json);
        }
        IStatement statement;
        try
        {
            statement = StatementParser.Parse(commandText);
        }
        catch (InvalidOperationException)
        {
            throw new DatasourceException($"Unsupported Memory command: {text}");
        }
        return statement switch
        {
            InsertStatement insert => Insert(insert, parameters),
            UpdateStatement update => Change(update.Table, parameters, jsonWhere ?? update.Where),
            DeleteStatement delete => Remove(delete.Table, parameters, jsonWhere ?? delete.Where),
            SelectStatement select => ReadSelect(select, parameters),
            _ => throw new DatasourceException($"Unsupported Memory command: {text}")
        };
    }

    Response Named(string op, IReadOnlyList<Parameter>? parameters, string? whereText)
    {
        var rows = GetCollection(Collection);
        var obj = Helpers.ParametersToRecord(parameters).ToDictionary(k => k.Key, v => v.Value);
        if (op == "insert")
            return InsertRow(obj, rows, Collection, parameters);
        if (op == "update")
        {
            var fromParam = obj.TryGetValue("$where", out var where) && where is Value.Text text ? text.Item : null;
            obj.Remove("$where");
            var filter = whereText is not null
                ? ParseFilter(whereText)
                : fromParam is not null
                    ? ParseFilter(fromParam)
                    : obj.TryGetValue("id", out var id)
                        ? new Row { ["id"] = id }
                        : new Row(obj);
            var updated = ApplyUpdate(rows, new Row(obj), filter);
            return new Response(updated.Result, updated.RowsAffected, parameters);
        }
        var deleted = ApplyDelete(rows, whereText is not null ? ParseFilter(whereText) : new Row(obj));
        return new Response(deleted.Result, deleted.RowsAffected, parameters);
    }

    static bool TryNamed(string text, string name, out string? rest)
    {
        var op = text.Trim();
        if (op.Equals(name, StringComparison.OrdinalIgnoreCase))
        {
            rest = null;
            return true;
        }
        var prefix = name + " ";
        if (op.StartsWith(prefix, StringComparison.OrdinalIgnoreCase))
        {
            var tail = op[prefix.Length..].Trim();
            if (LooksJson(tail))
            {
                rest = tail;
                return true;
            }
        }
        rest = null;
        return false;
    }

    Response Insert(InsertStatement insert, IReadOnlyList<Parameter>? parameters)
    {
        var rows = Rows(insert.Table, create: true);
        var obj = Helpers.ParametersToRecord(parameters).ToDictionary(k => k.Key, v => v.Value);
        return InsertRow(obj, rows, insert.Table.Length > 0 ? insert.Table : Collection, parameters);
    }

    Response InsertRow(Dictionary<string, Value> obj, List<Row> rows, string table, IReadOnlyList<Parameter>? parameters)
    {
        var id = InsertedId(obj, rows);
        obj["id"] = id;
        rows.Add(new Row(obj));
        inserted[table] = id;
        return new Response(id, 1, parameters);
    }

    Response Change(string table, IReadOnlyList<Parameter>? parameters, IWhere? where)
    {
        var rows = Rows(table, create: false);
        var obj = Helpers.ParametersToRecord(parameters).ToDictionary(k => k.Key, v => v.Value);
        return ApplyChanged(rows, obj, where, parameters);
    }

    Response Remove(string table, IReadOnlyList<Parameter>? parameters, IWhere? where)
    {
        var rows = Rows(table, create: false);
        var obj = Helpers.ParametersToRecord(parameters).ToDictionary(k => k.Key, v => v.Value);
        return RemoveMatched(rows, obj, where, parameters);
    }

    Response ReadSelect(SelectStatement select, IReadOnlyList<Parameter>? parameters)
    {
        var rows = Rows(select.Table, create: false);
        Records = select.Where is Where clause
            ? [.. rows.Where(row => BoundMatch(clause, row, parameters))]
            : [.. rows];
        return new Response(Value.Null.Instance, Records.Count, parameters);
    }

    List<Row> Rows(string? table, bool create) =>
        table is { Length: > 0 } name && name != "mem" && (create || HasCollection(name))
            ? GetCollection(name)
            : GetCollection(Collection);

    IReadOnlyList<Row> InsertedRow(string table, string column)
    {
        var id = inserted.TryGetValue(table, out var value) ? value : Value.Null.Instance;
        return id is Value.Null ? [] : [new Row { [column] = id }];
    }

    Response ApplyChanged(List<Row> rows, Dictionary<string, Value> obj, IWhere? where, IReadOnlyList<Parameter>? parameters)
    {
        if (where is Where clause)
        {
            var names = SetNames(parameters, where);
            var affected = 0;
            foreach (var row in rows)
            {
                if (!BoundMatch(clause, row, parameters)) continue;
                foreach (var name in names)
                    if (obj.TryGetValue(name, out var value)) row[name] = value;
                affected++;
            }
            return new Response(Value.Null.Instance, affected, parameters);
        }

        obj.Remove("$where");
        var filter = LooksJson(RenderWhere(where))
            ? ParseFilter(RenderWhere(where))
            : obj.TryGetValue("id", out var id) ? new Row { ["id"] = id } : new Row(obj);
        var updated = ApplyUpdate(rows, new Row(obj), filter);
        return new Response(updated.Result, updated.RowsAffected, parameters);
    }

    Response RemoveMatched(List<Row> rows, Dictionary<string, Value> obj, IWhere? where, IReadOnlyList<Parameter>? parameters)
    {
        if (where is Where clause)
        {
            var affected = rows.RemoveAll(row => BoundMatch(clause, row, parameters));
            return new Response(Value.Null.Instance, affected, parameters);
        }

        var filter = LooksJson(RenderWhere(where)) ? ParseFilter(RenderWhere(where)) : new Row(obj);
        var deleted = ApplyDelete(rows, filter);
        return new Response(deleted.Result, deleted.RowsAffected, parameters);
    }

    static bool BoundMatch(Where where, Row row, IReadOnlyList<Parameter>? parameters)
    {
        var bound = Helpers.ParametersToRecord(parameters);
        return MatchExpr(where.Condition, row, bound, parameters);
    }

    static bool MatchExpr(IExpression expression, Row row, Row bound, IReadOnlyList<Parameter>? parameters) =>
        expression switch
        {
            Operator.And(var left, var right) =>
                MatchExpr(left, row, bound, parameters) && MatchExpr(right, row, bound, parameters),
            Operator.Or(var left, var right) =>
                MatchExpr(left, row, bound, parameters) || MatchExpr(right, row, bound, parameters),
            Operator.Equals(Identifier name, _) =>
                row.ContainsKey(name.Name) && Equals(
                    row[name.Name],
                    bound.TryGetValue(name.Name, out var parameter) ? parameter : Value.Null.Instance),
            Operator.In(Identifier name, ArrayExpression arr) => InMatch(name.Name, arr, row, bound),
            Operator.NotIn(Identifier name, ArrayExpression arr) => !InMatch(name.Name, arr, row, bound),
            _ => new ExpressionEvaluator().Evaluate(expression, Context(row, parameters)) is Value.Bool(true)
        };

    static bool InMatch(string name, ArrayExpression arr, Row row, Row bound)
    {
        if (!row.TryGetValue(name, out var left)) return false;
        for (var i = 0; i < arr.Elements.Count; i++)
        {
            var value = arr.Elements[i] is Literal { Value: not Value.Null } literal
                ? literal.Value
                : bound.TryGetValue($"{name}_{i}", out var parameter) ? parameter : Value.Null.Instance;
            if (Equals(left, value)) return true;
        }
        return false;
    }

    static Row Context(Row row, IReadOnlyList<Parameter>? parameters)
    {
        var extras = Helpers.ParametersToRecord(parameters);
        var copy = new Row(row);
        foreach (var pair in extras)
            if (!copy.ContainsKey(pair.Key)) copy[pair.Key] = pair.Value;
        return copy;
    }

    static Response ApplyUpdate(List<Row> collection, Row obj, Row filter)
    {
        var affected = 0;
        foreach (var row in collection.Where(row => Matches(row, filter)))
        {
            foreach (var pair in obj) row[pair.Key] = pair.Value;
            affected++;
        }
        return new Response(Value.Null.Instance, affected);
    }

    static Response ApplyDelete(List<Row> collection, Row filter)
    {
        var affected = collection.RemoveAll(row => Matches(row, filter));
        return new Response(Value.Null.Instance, affected);
    }

    static IReadOnlyList<string> SetNames(IReadOnlyList<Parameter>? parameters, IWhere where)
    {
        var bound = where.Parameters.Select(parameter => parameter.Name).ToHashSet();
        return [.. (parameters ?? []).Select(parameter => parameter.Name).Where(name => !bound.Contains(name) && !name.EndsWith("__where", StringComparison.Ordinal))];
    }

    internal static Row ParseFilter(string? text) =>
        string.IsNullOrWhiteSpace(text)
            ? new Row()
            : JsonNode.Parse(text) is JsonObject obj
                ? new Row(obj.ToDictionary(k => k.Key, v => JsonValueToValue(v.Value)))
                : throw new DatasourceException("Memory filter must be an object");

    internal static bool Matches(Row row, Row filter) =>
        filter.All(pair => Equals(row.TryGetValue(pair.Key, out var value) ? value : Value.Null.Instance, pair.Value));

    static Row Merge(Row left, Row right)
    {
        foreach (var pair in right) left[pair.Key] = pair.Value;
        return left;
    }

    internal static Value JsonValueToValue(JsonNode? node)
    {
        if (node is not JsonValue value) return Value.From(node?.ToString());
        if (value.TryGetValue<int>(out var i)) return i;
        if (value.TryGetValue<long>(out var l)) return l;
        if (value.TryGetValue<bool>(out var b)) return b;
        if (value.TryGetValue<double>(out var d)) return d;
        if (value.TryGetValue<string>(out var s)) return s;
        return value.ToString();
    }

    static string? RenderWhere(IWhere? where) =>
        where is null ? null : DefaultStatementTextRenderer.Instance.Render(where);

    static bool LooksJson(string? text) => text?.TrimStart().StartsWith('{') == true;

    static bool TryJsonWhere(string text, out string sql, out string json)
    {
        var mark = text.LastIndexOf(" WHERE ", StringComparison.OrdinalIgnoreCase);
        if (mark < 0)
        {
            sql = text;
            json = "";
            return false;
        }
        var tail = text[(mark + 7)..].Trim();
        if (!LooksJson(tail))
        {
            sql = text;
            json = "";
            return false;
        }
        sql = text[..mark];
        json = tail;
        return true;
    }

    static Value InsertedId(Dictionary<string, Value> row, List<Row> rows) =>
        row.TryGetValue("id", out var existing) && existing is Value.Int
            ? existing
            : rows.Select(item => item.TryGetValue("id", out var cell) && cell is Value.Int id ? id.Item : 0L).DefaultIfEmpty(0).Max() + 1;

    static List<Row> CloneRows(IEnumerable<Row> rows) => rows.Select(AsRow).ToList();

    static Row AsRow(IReadOnlyDictionary<string, Value> row) => new(row);
}
