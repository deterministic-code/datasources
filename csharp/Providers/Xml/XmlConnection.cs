using Datasources;

namespace Datasources.Xml;

internal sealed class XmlConnection : StubConnection
{
    public XmlConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("xml") { }
    public override ICommand CreateCommand() => new XmlCommand(this);

}
