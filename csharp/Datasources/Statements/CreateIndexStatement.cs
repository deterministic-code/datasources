namespace Datasources;

public sealed record CreateIndexStatement : ICreateIndexStatement
{
    public string Name { get; }
    public string Table { get; }
    public IReadOnlyList<string> Columns { get; }
    public bool Unique { get; }

    public CreateIndexStatement(string name, string table, IReadOnlyList<string> columns, bool unique = false) =>
        (Name, Table, Columns, Unique) = (name, table, columns, unique);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public static string NameOf(string entity, StatementNaming naming, params string[] parts) =>
        ConstraintName.Of(entity, naming, parts);
}
