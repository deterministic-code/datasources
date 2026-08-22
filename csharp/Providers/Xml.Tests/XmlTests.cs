using Datasources;
using Datasources.Xml;

namespace Datasources.Xml.Tests;

public class XmlTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(XmlProvider.Factory);
        Assert.Equal("xml", XmlProvider.Factory.Provider);
        var ds = Datasource.Create("xml");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
