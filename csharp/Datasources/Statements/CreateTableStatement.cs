namespace Datasources;

public sealed record CreateTableStatement : ICreateTableStatement
{
    readonly string? text;
    public string? Table { get; }
    public IReadOnlyList<Column> Columns { get; }
    public IReadOnlyList<ITableConstraint> Constraints { get; }

    public CreateTableStatement(string text)
    {
        this.text = text;
        Columns = [];
        Constraints = [];
    }

    public CreateTableStatement(
        string table,
        IReadOnlyList<Column> columns,
        IReadOnlyList<ITableConstraint>? constraints = null)
    {
        Table = table;
        Columns = columns;
        Constraints = constraints ?? [];
    }

    public string GetText() => text ?? new CreateTableStatementRenderer().Render(this);
}
