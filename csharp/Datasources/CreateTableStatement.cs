namespace Datasources;

public sealed class CreateTableStatement : ICreateTableStatement
{
    private readonly string _text;
    public CreateTableStatement(string text) => _text = text;
    public string GetText() => _text;
}
