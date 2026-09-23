namespace Datasources;

public sealed record ExecuteStoredProcedureStatement : IExecuteStoredProcedureStatement
{
    readonly string text;
    public ExecuteStoredProcedureStatement(string text) => (this.text, Parameters) = (text, []);
    public ExecuteStoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters) =>
        (text, Parameters) = ($"CALL {name}({Args(parameters)})", parameters);
    public IReadOnlyList<Parameter> Parameters { get; }
    public string GetText() => text;
    static string Args(IReadOnlyList<Parameter> parameters) =>
        string.Join(", ", parameters.Where(p => p.Direction != ParameterDirection.ReturnValue).Select(p => $"${p.Name}"));
}
