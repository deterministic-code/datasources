using Datasources;

namespace Datasources.Text;

public sealed class TextCommand : StubCommand
{
    public TextCommand(IConnection connection) : base("text", connection) { }
}
