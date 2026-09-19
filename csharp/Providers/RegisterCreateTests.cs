using Datasources;

public partial class RegisterCreateTests
{
    public RegisterCreateTests() => DatasourceFactory.Reset();

    [Fact]
    public void Creates()
    {
        DatasourceFactory.Register(Provider, Connection, Repository);
        Assert.Equal(
            Provider,
            DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = Provider }).Provider);
    }
}
