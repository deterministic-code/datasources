namespace Datasources;

public sealed record InsertStatement : IInsertStatement
{
    readonly string text;
    public InsertStatement(string text) => this.text = text;
    public InsertStatement(string table, IReadOnlyList<Parameter> fields) =>
        text = $"INSERT INTO {table} ({string.Join(", ", fields.Select(field => field.Name))}) VALUES ({string.Join(", ", fields.Select(field => $"${field.Name}"))})";
    public string GetText() => text;
}
