namespace Datasources;

public sealed record RebuildTableStatement : IRebuildTableStatement
{
    public string Table { get; }
    public CreateTableStatement Create { get; }
    public IReadOnlyList<RebuildColumn> Columns { get; }
    public IReadOnlyList<IStatement> Recreate { get; }

    public RebuildTableStatement(
        string table,
        CreateTableStatement create,
        IReadOnlyList<RebuildColumn> columns,
        IReadOnlyList<IStatement>? recreate = null)
    {
        Table = table;
        Create = create;
        Columns = columns;
        Recreate = recreate ?? [];
    }

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
