namespace Datasources;

public sealed record BulkUpdateStatement : IBulkUpdateStatement
{
    public string Table { get; }
    public IReadOnlyList<string> Set { get; }
    public IReadOnlyList<string> Ids { get; }
    public IReadOnlyList<(IReadOnlyList<Parameter> Fields, IReadOnlyList<Parameter> Ids)> Rows { get; }

    public BulkUpdateStatement(
        string table,
        IReadOnlyList<string> set,
        IReadOnlyList<string> ids,
        IReadOnlyList<(IReadOnlyList<Parameter> Fields, IReadOnlyList<Parameter> Ids)> rows) =>
        (Table, Set, Ids, Rows) = (table, set, ids, rows);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public IReadOnlyList<Parameter> Parameters =>
        BulkInsertStatement.Flatten([.. Set, .. Ids], Rows.Select(row => row.Fields.Concat(row.Ids).ToList()));
}
