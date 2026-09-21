namespace Datasources;

public sealed record StoredProcedureStatement : IStoredProcedureStatement
{
    readonly string text;
    public StoredProcedureStatement(string text) => this.text = text;
    public StoredProcedureStatement(string name, IReadOnlyList<Parameter> parameters) =>
        text = $"CALL {name}({string.Join(", ", parameters.Select(field => $"${field.Name}"))})";
    public string GetText() => text;
}
