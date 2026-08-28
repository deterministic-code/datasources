namespace Datasources;

public sealed class MockQuery : IQuery
{
    public int GetTextCalls { get; private set; }
    public MockQuery(string text, IWhere? where = null) => (Text, Where) = (text, where);
    public string Text { get; set; }
    public IWhere? Where { get; set; }
    public string GetText()
    {
        GetTextCalls++;
        return Text;
    }
}
