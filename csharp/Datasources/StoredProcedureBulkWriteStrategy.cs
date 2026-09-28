using System.Globalization;
using System.Text;

namespace Datasources;

public sealed class StoredProcedureBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
{
    readonly string create;
    readonly string update;
    readonly string delete;

    public StoredProcedureBulkWriteStrategy(string create, string update, string delete) =>
        (this.create, this.update, this.delete) = (create, update, delete);

    public Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo) =>
        Call(repo, create, items);

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo) =>
        Call(repo, update, items);

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo) =>
        Call(repo, delete, items);

    static Task<Response> Call(BulkCRUDRepository<T> repo, string name, IReadOnlyList<T> items)
    {
        if (items.Count == 0)
            return Task.FromResult(new Response(Value.Null.Instance, 0));
        var payload = new Parameter("payload", Json(repo.Rows(items)));
        return repo.Command().Execute(new ExecuteStoredProcedureStatement(name, [payload]), [payload]);
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
