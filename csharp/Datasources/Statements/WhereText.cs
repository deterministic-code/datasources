namespace Datasources;

public sealed record WhereText : IWhere
{
    readonly string text;
    public WhereText(string text) => this.text = text;
    public string GetText() => text;
}
