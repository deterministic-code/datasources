using Datasources;

namespace Datasources.Xml;

public sealed class XmlDatasource : IDatasourceFactory
{
    public static readonly XmlDatasource Factory = new();
    public string Provider => "xml";
    public IDatasource Create(object? options = null) => new StubDatasource("xml");
}
