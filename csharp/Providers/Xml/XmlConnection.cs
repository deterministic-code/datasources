using Datasources;

namespace Datasources.Xml;

public sealed class XmlConnection : StubConnection
{
    public XmlConnection() : base("xml") { }
}
