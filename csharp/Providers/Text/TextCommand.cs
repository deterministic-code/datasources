using Datasources;

namespace Datasources.Text;

internal sealed class TextCommand : StubCommand
{
    public TextCommand(IConnection connection) : base("text", connection) { }
}
