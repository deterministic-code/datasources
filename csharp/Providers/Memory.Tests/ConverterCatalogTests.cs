using Datasources;
using Datasources.Memory;

namespace Datasources.Memory.Tests;

public class ConverterCatalogTests
{
    [Fact]
    public void EmptyConverters()
    {
        IDriver driver = new MemoryDriver();
        Assert.Null(driver.TypeConverter);
        Assert.Empty(driver.RowTypeConverters);
        Assert.Same(EmptyDefaultValueConverter.Instance, driver.DefaultValueConverter);
        Assert.Null(driver.DefaultValueConverter.Convert("string", "hi"));
    }
}
