namespace Datasources;

public sealed record CreateTableStatement : ICreateTableStatement
{
    readonly string text;
    public CreateTableStatement(string text) => this.text = text;
    public string GetText() => text;
}
