namespace Datasources;

public sealed record InsertStatement : IInsertStatement
{
    readonly string? text;
    public string? Table { get; }
    public IReadOnlyList<Parameter> Fields { get; } = [];

    public InsertStatement(string text) => this.text = text;

    public InsertStatement(string table, IReadOnlyList<Parameter> fields) =>
        (Table, Fields) = (table, fields);

    public string GetText() => text ??
        $"INSERT INTO {Table} ({string.Join(", ", Fields.Select(field => field.Name))}) VALUES ({string.Join(", ", Fields.Select(field => $"${field.Name}"))})";
}
