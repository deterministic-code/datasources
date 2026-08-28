namespace Datasources;

public sealed class UpdateStatement : IUpdateStatement
{
    private readonly string _text;
    public UpdateStatement(string text, IWhere? where = null) => (_text, Where) = (text, where);
    public IWhere? Where { get; }
    public string GetText() => _text;
}
