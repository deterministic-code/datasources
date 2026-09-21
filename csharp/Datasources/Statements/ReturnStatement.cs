namespace Datasources;

public sealed record ReturnStatement : IReturnStatement
{
    readonly string text;
    public ReturnStatement(string text) => this.text = text;
    public ReturnStatement(string name, IReadOnlyList<Parameter> parameters) =>
        text = $"SELECT {name}({Args(parameters)})";
    public string GetText() => text;
    static string Args(IReadOnlyList<Parameter> parameters) =>
        string.Join(", ", parameters.Where(p => p.Direction != ParameterDirection.ReturnValue).Select(p => $"${p.Name}"));
}
