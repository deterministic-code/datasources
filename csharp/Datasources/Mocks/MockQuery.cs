namespace Datasources;

public sealed class MockQuery : IQuery
{
    public int GetTextCalls { get; private set; }
    public MockQuery(string text) => Text = text;
    public string Text { get; set; }
    public string GetText()
    {
        GetTextCalls++;
        return Text;
    }
}
