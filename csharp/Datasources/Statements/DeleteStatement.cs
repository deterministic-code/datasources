namespace Datasources;

public sealed record DeleteStatement : IDeleteStatement
{
    public string Table { get; }
    public IWhere? Where { get; }

    public DeleteStatement(string table, IWhere? where = null) =>
        (Table, Where) = (table, where);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
