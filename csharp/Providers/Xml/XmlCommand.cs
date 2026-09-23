using Datasources;

namespace Datasources.Xml;

internal sealed class XmlCommand : StubCommand
{
    public XmlCommand(IConnection connection) : base("xml", connection) { }
}
