namespace Datasources;

public sealed record Query : IQuery
{
    readonly string text;
    public Query(string text, IWhere? where = null) => (this.text, Where) = (text, where);
    public IWhere? Where { get; }
    public string GetText() => text;
}
