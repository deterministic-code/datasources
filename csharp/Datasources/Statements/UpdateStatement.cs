namespace Datasources;

public sealed record UpdateStatement : IUpdateStatement
{
    public string Table { get; }
    public IReadOnlyList<Parameter> Fields { get; } = [];
    public IWhere? Where { get; }

    public UpdateStatement(string table, IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        (Table, Fields, Where) = (table, fields, where);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public IReadOnlyList<Parameter> Parameters =>
        new UpdateStatementTextRenderer(DefaultStatementTextRenderer.Instance)
            .Compose($"UPDATE {Table} SET {string.Join(", ", Fields.Select(item => $"{item.Name} = ${item.Name}"))}", Where, Fields)
            .Parameters;

    public static IReadOnlyList<Parameter> ParameterList(IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        new UpdateStatementTextRenderer(DefaultStatementTextRenderer.Instance).Compose("", where, fields).Parameters;
}
