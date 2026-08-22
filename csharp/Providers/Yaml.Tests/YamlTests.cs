using Datasources;
using Datasources.Yaml;

namespace Datasources.Yaml.Tests;

public class YamlTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(YamlProvider.Factory);
        Assert.Equal("yaml", YamlProvider.Factory.Provider);
        var ds = Datasource.Create("yaml");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
