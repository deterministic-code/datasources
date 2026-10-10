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

    public Task<Response> Create(IReadOnlyList<T> items) => Then(Call(create, items), items, WriteMode.Create);

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere) => Then(Call(update, items), items, WriteMode.Update);

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere) => Then(Call(delete, items), items, WriteMode.Delete);

    async Task<Response> Then(Task<Response> write, IReadOnlyList<T> items, WriteMode mode)
    {
        var response = await write;
        await BulkWriteEnrichment.After(Repo, Options, items, mode);
        return response;
    }

    Task<Response> Call(string name, IReadOnlyList<T> items)
    {
        if (items.Count == 0)
            return Task.FromResult(new Response(Value.Null.Instance, 0));
        var payload = new Parameter("payload", Json([.. items.Select(RowOf)]));
        return Command.Execute(new ExecuteStoredProcedureStatement(name, [payload]), [payload]);
    }

    Row RowOf(T item)
    {
        var stored = Reverse(item);
        var builder = new CRUDRepositoryBuilder<T>();
        builder.Parts(Command.Connection, Options);
        var row = new Row();
        foreach (var name in builder.CRUDStatements?.ReadColumns ?? [])
        {
            var match = stored.FirstOrDefault(pair => Column(pair.Key) == name);
            row[name] = match.Key is null ? Value.Null.Instance : match.Value;
        }
        return row;
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
