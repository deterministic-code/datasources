using Datasources;
using Datasources.Memory;

public partial class RegisterCreateTests
{
    const string Provider = "memory";
    static readonly IDriver Driver = new MemoryProvider();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(MemoryProvider), Assert.Single(typeof(MemoryProvider).Assembly.GetExportedTypes()));
}
