namespace Datasources;

public sealed record StoredProcedureStatement : IStoredProcedureStatement
{
    readonly string text;
    public StoredProcedureStatement(string text) => this.text = text;
    public string GetText() => text;
}
