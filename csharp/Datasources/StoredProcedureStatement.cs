namespace Datasources;

public sealed class StoredProcedureStatement : IStoredProcedureStatement
{
    private readonly string _text;
    public StoredProcedureStatement(string text) => _text = text;
    public string GetText() => _text;
}
