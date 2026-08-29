using Datasources;

public partial class RegisterCreateTests
{
    public RegisterCreateTests() => Datasource.Reset();

    [Fact]
    public void Creates()
    {
        Datasource.Use(Instance);
        Assert.Equal(Provider, Datasource.Create(Provider).Provider);
    }
}
