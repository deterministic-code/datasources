using Datasources;
using Datasources.Memory;

public partial class RegisterCreateTests
{
    const string Provider = "memory";
    static readonly IDriver Driver = new MemoryDriver();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(MemoryDriver), Assert.Single(typeof(MemoryDriver).Assembly.GetExportedTypes()));
}
