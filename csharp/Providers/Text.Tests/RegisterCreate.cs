using Datasources;
using Datasources.Text;

public partial class RegisterCreateTests
{
    const string Provider = "text";
    static readonly IDatasourceFactory Factory = TextDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new TextDatasource().CreateConnection();
        Assert.IsType<TextConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
