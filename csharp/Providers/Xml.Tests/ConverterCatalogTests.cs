using Datasources;
using Datasources.Xml;

public class ConverterCatalogTests
{
    [Fact]
    public void EmptyConverters()
    {
        IDriver driver = new XmlDriver();
        Assert.Null(driver.TypeConverter);
        Assert.Empty(driver.RowTypeConverters);
        Assert.Same(EmptyDefaultValueConverter.Instance, driver.DefaultValueConverter);
    }
}
