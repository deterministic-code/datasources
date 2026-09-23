using Datasources;
using Datasources.Json;

public partial class RegisterCreateTests
{
    const string Provider = "json";
    static readonly IDriver Driver = new JsonProvider();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(JsonProvider), Assert.Single(typeof(JsonProvider).Assembly.GetExportedTypes()));
}
