namespace Datasources;

public sealed record ExecuteStoredProcedureStatement : IExecuteStoredProcedureStatement
{
    readonly string text;
    public ExecuteStoredProcedureStatement(string text) => this.text = text;
    public ExecuteStoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters) =>
        text = $"CALL {name}({Args(parameters)})";
    public string GetText() => text;
    static string Args(IReadOnlyList<Parameter> parameters) =>
        string.Join(", ", parameters.Where(p => p.Direction != ParameterDirection.ReturnValue).Select(p => $"${p.Name}"));
}
