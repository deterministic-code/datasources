using Datasources;
using Datasources.Json;

public partial class RegisterCreateTests
{
    const string Provider = "json";
    static readonly IDriver Driver = new JsonDriver();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(JsonDriver), Assert.Single(typeof(JsonDriver).Assembly.GetExportedTypes()));
}
