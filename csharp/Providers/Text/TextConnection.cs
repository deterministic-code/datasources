using Datasources;

namespace Datasources.Text;

internal sealed class TextConnection : StubConnection
{
    public TextConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("text") { }
    public override ICommand CreateCommand() => new TextCommand(this);

}
