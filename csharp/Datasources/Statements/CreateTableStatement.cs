namespace Datasources;

public sealed record CreateTableStatement : ICreateTableStatement
{
    public string Table { get; }
    public IReadOnlyList<Column> Columns { get; }
    public IReadOnlyList<ITableConstraint> Constraints { get; }

    public CreateTableStatement(
        string table,
        IReadOnlyList<Column> columns,
        IReadOnlyList<ITableConstraint>? constraints = null)
    {
        Table = table;
        Columns = columns;
        Constraints = constraints ?? [];
    }

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
