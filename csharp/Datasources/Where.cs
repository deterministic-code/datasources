namespace Datasources;

public sealed class Where : IWhere
{
    private readonly string _text;
    public Where(string text) => _text = text;
    public string GetText() => _text;
}
