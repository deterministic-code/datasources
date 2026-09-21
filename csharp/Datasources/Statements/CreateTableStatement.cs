namespace Datasources;

public sealed record CreateTableStatement : ICreateTableStatement
{
    readonly string text;
    public CreateTableStatement(string text) => this.text = text;
    public CreateTableStatement(string table, IReadOnlyList<Column> columns) =>
        text = $"CREATE TABLE {table} ({string.Join(", ", columns.Select(column => $"{column.Name} {column.Type}"))})";
    public string GetText() => text;
}
