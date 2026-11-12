using Datasources;
using Datasources.Yaml;

public class ConverterCatalogTests
{
    [Fact]
    public void EmptyConverters()
    {
        IDriver driver = new YamlDriver();
        Assert.Null(driver.TypeConverter);
        Assert.Empty(driver.RowTypeConverters);
        Assert.Same(EmptyDefaultValueConverter.Instance, driver.DefaultValueConverter);
    }
}
