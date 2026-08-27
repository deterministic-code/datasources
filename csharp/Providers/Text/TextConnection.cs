using Datasources;

namespace Datasources.Text;

public sealed class TextConnection : StubConnection
{
    public TextConnection() : base("text") { }
}
