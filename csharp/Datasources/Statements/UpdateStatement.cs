namespace Datasources;

public sealed record UpdateStatement : IUpdateStatement
{
    readonly string text;
    public UpdateStatement(string text, IWhere? where = null) => (this.text, Where) = (text, where);
    public IWhere? Where { get; }
    public string GetText() => text;
}
