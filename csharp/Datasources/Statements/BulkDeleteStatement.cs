namespace Datasources;

public sealed record BulkDeleteStatement : IBulkDeleteStatement
{
    public string Table { get; }
    public IReadOnlyList<string> Ids { get; }
    public IReadOnlyList<IReadOnlyList<Parameter>> Rows { get; }

    public BulkDeleteStatement(string table, IReadOnlyList<string> ids, IReadOnlyList<IReadOnlyList<Parameter>> rows) =>
        (Table, Ids, Rows) = (table, ids, rows);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public IReadOnlyList<Parameter> Parameters => BulkInsertStatement.Flatten(Ids, Rows);
}
