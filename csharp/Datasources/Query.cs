namespace Datasources;

public sealed class Query : IQuery
{
    private readonly string _text;
    public Query(string text) => _text = text;
    public string GetText() => _text;
}
