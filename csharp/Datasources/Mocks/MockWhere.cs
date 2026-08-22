namespace Datasources;

public sealed class MockWhere : IWhere
{
    public int GetTextCalls { get; private set; }
    public MockWhere(string text) => Text = text;
    public string Text { get; set; }
    public string GetText()
    {
        GetTextCalls++;
        return Text;
    }
}
