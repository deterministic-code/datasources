using Datasources;
using Datasources.Xml;

public partial class RegisterCreateTests
{
    const string Provider = "xml";
    static readonly IDatasourceFactory Factory = XmlDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new XmlDatasource().CreateConnection();
        Assert.IsType<XmlConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
