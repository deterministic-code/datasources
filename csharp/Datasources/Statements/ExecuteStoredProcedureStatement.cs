namespace Datasources;

public sealed record ExecuteStoredProcedureStatement : IExecuteStoredProcedureStatement
{
    public string Name { get; }
    public IReadOnlyList<Parameter> Parameters { get; }

    public ExecuteStoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters) =>
        (Name, Parameters) = (name, parameters);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
