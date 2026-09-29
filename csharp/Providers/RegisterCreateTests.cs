using Datasources;

public partial class RegisterCreateTests
{
    public RegisterCreateTests() => DatasourceFactory.Reset();

    [Fact]
    public void Creates()
    {
        DatasourceFactory.Register(Provider, Driver);
        Assert.Equal(
            Provider,
            DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = Provider }).Provider);
    }
}
