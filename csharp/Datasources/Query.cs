namespace Datasources;

public sealed class Query : IQuery
{
    private readonly string _text;
    public Query(string text, IWhere? where = null) => (_text, Where) = (text, where);
    public IWhere? Where { get; }
    public string GetText() => _text;
}
