namespace Datasources;

public sealed record CreateIndexStatement : ICreateIndexStatement
{
    readonly string text;
    public CreateIndexStatement(string text) => this.text = text;
    public CreateIndexStatement(string name, string table, IReadOnlyList<string> columns, bool unique = false) =>
        text = $"CREATE {(unique ? "UNIQUE " : "")}INDEX {name} ON {table} ({string.Join(", ", columns)})";
    public string GetText() => text;
}
