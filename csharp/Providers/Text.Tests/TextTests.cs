using Datasources;
using Datasources.Text;

namespace Datasources.Text.Tests;

public class TextTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(TextProvider.Factory);
        Assert.Equal("text", TextProvider.Factory.Provider);
        var ds = Datasource.Create("text");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
