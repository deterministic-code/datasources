using Datasources;

namespace Datasources.Xml;

public sealed class XmlCommand : StubCommand
{
    public XmlCommand(IConnection connection) : base("xml", connection) { }
}
