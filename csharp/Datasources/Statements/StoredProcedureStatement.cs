namespace Datasources;

public sealed record StoredProcedureStatement : IStoredProcedureStatement
{
    readonly string text;
    public StoredProcedureStatement(string text) => this.text = text;
    public StoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters) =>
        text = $"CALL {name}({Args(parameters)})";
    public string GetText() => text;
    static string Args(IReadOnlyList<Parameter> parameters) =>
        string.Join(", ", parameters.Where(p => p.Direction != ParameterDirection.ReturnValue).Select(p => $"${p.Name}"));
}
