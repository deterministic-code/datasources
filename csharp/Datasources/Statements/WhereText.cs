namespace Datasources;

public sealed record WhereText : IWhere
{
    readonly string text;
    public WhereText(string text) => this.text = text;
    internal string RawText => text;
    public string Accept(IConditionVisitor visitor) => visitor.Visit(this);
}
