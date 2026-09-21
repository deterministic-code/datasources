namespace Datasources;

public sealed record Where : IWhere
{
    readonly string text;
    public Where(string text) => this.text = text;
    public string GetText() => text;
}
