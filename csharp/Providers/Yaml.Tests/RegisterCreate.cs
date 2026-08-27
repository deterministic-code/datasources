using Datasources;
using Datasources.Yaml;

public partial class RegisterCreateTests
{
    const string Provider = "yaml";
    static readonly IDatasourceFactory Factory = YamlDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new YamlDatasource().CreateConnection();
        Assert.IsType<YamlConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
