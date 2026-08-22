using Datasources;

namespace Datasources.Xml;

public static class XmlProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("xml");

    static XmlProvider()
    {
        Datasource.Use(Factory);
    }
}
