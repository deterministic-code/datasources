using System.Globalization;
using System.Text;

namespace Datasources;

public sealed class StoredProcedureBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
{
    readonly string create;
    readonly string update;
    readonly string delete;

    public CRUDRepository<T> Repo { get; set; } = null!;
    public ICommand Command { get; set; } = null!;
    public CRUDRepositoryOptions<T> Options { get; set; } = null!;

    public void Attach(ICommand command, CRUDRepositoryOptions<T> options) =>
        (Command, Options) = (command, options);

    public StoredProcedureBulkWriteStrategy(string create, string update, string delete) =>
        (this.create, this.update, this.delete) = (create, update, delete);

    public Task<Response> Create(IReadOnlyList<T> items) => Then(Call(create, items, generated: true), items, WriteMode.Create);

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere) => Then(Call(update, items), items, WriteMode.Update);

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere) => Then(Call(delete, items), items, WriteMode.Delete);

    async Task<Response> Then(Task<Response> write, IReadOnlyList<T> items, WriteMode mode)
    {
        var response = await write;
        await BulkWriteEnrichment.After(Repo, Options, items, mode);
        return response;
    }

    async Task<Response> Call(string name, IReadOnlyList<T> items, bool generated = false)
    {
        if (items.Count == 0)
            return new Response(Value.Null.Instance, 0);
        var payload = new Parameter("payload", Json([.. items.Select(item => RowOf(item, generated))]));
        IReadOnlyList<Parameter> parameters = generated && WantsIds(items)
            ? [payload, new Parameter("ids", ParameterDirection.InputOutput)]
            : [payload];
        var response = await Command.Execute(new ExecuteStoredProcedureStatement(name, parameters), parameters);
        if (generated) ApplyIds(items, response.Parameters);
        return response;
    }

    Row RowOf(T item, bool create)
    {
        var stored = Reverse(item);
        var builder = new CRUDRepositoryBuilder<T>();
        builder.Parts(Command.Connection, Options);
        var row = new Row();
        foreach (var name in builder.CRUDStatements?.ReadColumns ?? [])
        {
            if (create && Identity(name) && !Assigned(stored, name)) continue;
            var match = stored.FirstOrDefault(pair => Column(pair.Key) == name);
            row[name] = match.Key is null ? Value.Null.Instance : match.Value;
        }
        return row;
    }

    bool Identity(string column) => Options.Ids.Count == 1 && Options.Ids[0] == column;

    bool WantsIds(IReadOnlyList<T> items) =>
        Options.Ids.Count == 1 && items.Select(Reverse).Any(row => !Assigned(row, Options.Ids[0]));

    static bool Assigned(Row record, string col) =>
        record.TryGetValue(col, out var value) && value switch
        {
            Value.Null => false,
            Value.Text(var text) => text.Length > 0,
            Value.Int(0) => false,
            _ => true
        };

    void ApplyIds(IReadOnlyList<T> items, IReadOnlyList<Parameter>? parameters)
    {
        if (Options.Ids.Count != 1) return;
        var found = parameters?.FirstOrDefault(parameter => parameter.Name == "ids");
        var ids = ParseIds(found?.Value);
        var column = Options.Ids[0];
        for (var index = 0; index < items.Count && index < ids.Count; index++)
            Stamp(items[index], column, ids[index]);
    }

    static IReadOnlyList<long> ParseIds(Value? value)
    {
        var text = value switch
        {
            Value.Text(var raw) => raw.Trim(),
            Value.Int(var number) => number.ToString(CultureInfo.InvariantCulture),
            Value.Binary(var bytes) => Encoding.UTF8.GetString(bytes).Trim(),
            _ => value?.ToObject()?.ToString()?.Trim() ?? ""
        };
        if (text is "" or "[]" or "null") return [];
        return [.. text.TrimStart('[').TrimEnd(']').Split(',', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(part => long.Parse(part, CultureInfo.InvariantCulture))];
    }

    static void Stamp(T item, string column, long id)
    {
        var property = typeof(T).GetProperty(column) ?? typeof(T).GetProperty("Id");
        if (property is not { CanWrite: true }) return;
        var type = Nullable.GetUnderlyingType(property.PropertyType) ?? property.PropertyType;
        property.SetValue(item, Convert.ChangeType(id, type, CultureInfo.InvariantCulture));
    }

    string Column(string property) =>
        Options.PropertyMappings?.Map(property) ?? property;

    Row Reverse(T item)
    {
        var objects = Options.ObjectConverter ?? new MetaDataRowTypeConverter<T> { ShouldCoerceId = true };
        var row = objects.ReverseConvert(item);
        return Options.RowConverter?.ReverseConvert(row) ?? row;
    }

    static string Json(IReadOnlyList<Row> rows) =>
        "[" + string.Join(",", rows.Select(Object)) + "]";

    static string Object(Row row) =>
        "{" + string.Join(",", row.OrderBy(pair => pair.Key, StringComparer.Ordinal).Select(pair => $"{Quote(pair.Key)}:{Scalar(pair.Value)}")) + "}";

    static string Scalar(Value value) => value switch
    {
        Value.Null => "null",
        Value.Bool(var flag) => flag ? "true" : "false",
        Value.Int(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Float(var number) => number.ToString("G17", CultureInfo.InvariantCulture),
        Value.Text(var text) => Quote(text),
        Value.DateTime(var date) => Quote(date.ToString("o", CultureInfo.InvariantCulture)),
        Value.Guid(var guid) => Quote(guid.ToString("D")),
        Value.Decimal(var number) => Quote(number.ToString(CultureInfo.InvariantCulture)),
        Value.Binary(var bytes) => Quote(Convert.ToBase64String(bytes)),
        _ => throw new DatasourceException("Unsupported bulk payload value")
    };

    static string Quote(string text)
    {
        var result = new StringBuilder("\"");
        foreach (var ch in text)
            result.Append(ch switch
            {
                '\\' => "\\\\",
                '"' => "\\\"",
                '\b' => "\\b",
                '\f' => "\\f",
                '\n' => "\\n",
                '\r' => "\\r",
                '\t' => "\\t",
                < ' ' => "\\u" + ((int)ch).ToString("x4"),
                _ => ch.ToString()
            });
        return result.Append('"').ToString();
    }
}
