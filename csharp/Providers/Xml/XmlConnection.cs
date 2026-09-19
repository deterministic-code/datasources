using Datasources;

namespace Datasources.Xml;

public sealed class XmlConnection : StubConnection
{
    public XmlConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("xml") { }
    public override ICommand CreateCommand() => new XmlCommand(this);
    public static object Repository(IConnection connection) =>
        new StubRepository<object>(connection.Provider, connection);
}
