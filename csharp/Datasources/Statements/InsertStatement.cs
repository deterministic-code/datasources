namespace Datasources;

public sealed record InsertStatement : IInsertStatement
{
    readonly string text;
    public InsertStatement(string text) => this.text = text;
    public string GetText() => text;
}
