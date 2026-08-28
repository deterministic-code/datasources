namespace Datasources;

public sealed class DeleteStatement : IDeleteStatement
{
    private readonly string _text;
    public DeleteStatement(string text, IWhere? where = null) => (_text, Where) = (text, where);
    public IWhere? Where { get; }
    public string GetText() => _text;
}
