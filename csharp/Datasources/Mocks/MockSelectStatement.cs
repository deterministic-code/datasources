namespace Datasources;

public sealed class MockSelectStatement : ISelectStatement
{
    public int AcceptCalls { get; private set; }
    public MockSelectStatement(string text, IWhere? where = null) => (Text, Where) = (text, where);
    public string Text { get; set; }
    public IWhere? Where { get; set; }
    public string Accept(IStatementVisitor visitor)
    {
        AcceptCalls++;
        return Text;
    }
}
