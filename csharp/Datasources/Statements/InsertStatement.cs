namespace Datasources;

public sealed record InsertStatement : IInsertStatement
{
    public string Table { get; }
    public IReadOnlyList<Parameter> Fields { get; } = [];

    public InsertStatement(string table, IReadOnlyList<Parameter> fields) =>
        (Table, Fields) = (table, fields);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
