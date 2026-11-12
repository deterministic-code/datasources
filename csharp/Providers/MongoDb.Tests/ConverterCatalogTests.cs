using Datasources;
using Datasources.MongoDb;

public class ConverterCatalogTests
{
    [Fact]
    public void EmptyConverters()
    {
        IDriver driver = new MongoDbDriver();
        Assert.Null(driver.TypeConverter);
        Assert.Empty(driver.RowTypeConverters);
        Assert.Same(EmptyDefaultValueConverter.Instance, driver.DefaultValueConverter);
    }
}
