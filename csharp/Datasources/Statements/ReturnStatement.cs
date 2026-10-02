namespace Datasources;

public sealed record ReturnStatement : IReturnStatement
{
    public string Name { get; }
    public IReadOnlyList<Parameter> Parameters { get; }

    public ReturnStatement(string name, IReadOnlyList<Parameter> parameters) =>
        (Name, Parameters) = (name, parameters);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
