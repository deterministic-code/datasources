namespace Datasources;

public sealed class InsertStatement : IInsertStatement
{
    private readonly string _text;
    public InsertStatement(string text) => _text = text;
    public string GetText() => _text;
}
