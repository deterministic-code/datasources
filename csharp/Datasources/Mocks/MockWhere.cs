namespace Datasources;

public sealed class MockWhere : IWhere
{
    public int AcceptCalls { get; private set; }
    public MockWhere(string text) => Text = text;
    public string Text { get; set; }
    public string Accept(IConditionVisitor visitor)
    {
        AcceptCalls++;
        return Text;
    }
}
