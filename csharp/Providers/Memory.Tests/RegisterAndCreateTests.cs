using Datasources;
using Datasources.Memory;

namespace Datasources.Memory.Tests;

public class RegisterAndCreateTests
{
    public RegisterAndCreateTests() => DatasourceFactory.Reset();

    [Fact]
    public void RegisterAndCreate()
    {
        DatasourceFactory.Register("memory", new MemoryDriver());
        var ds = DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "memory" });
        Assert.Equal("memory", ds.Provider);
    }
}
