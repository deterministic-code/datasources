using Datasources;

namespace Datasources.Text;

public sealed class TextConnection : StubConnection
{
    public TextConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("text") { }
    public override ICommand CreateCommand() => new TextCommand(this);
    public static object Repository(IConnection connection) =>
        new StubRepository<object>(connection.Provider, connection);
}
