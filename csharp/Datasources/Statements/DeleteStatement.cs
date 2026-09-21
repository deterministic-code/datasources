namespace Datasources;

public sealed record DeleteStatement : IDeleteStatement
{
    readonly string text;
    public DeleteStatement(string text, IWhere? where = null) => (this.text, Where) = (text, where);
    public IWhere? Where { get; }
    public string GetText() => text;
}
