using Datasources;
using Datasources.DynamoDb;

public class ConverterCatalogTests
{
    [Fact]
    public void EmptyConverters()
    {
        IDriver driver = new DynamoDbDriver();
        Assert.Null(driver.TypeConverter);
        Assert.Empty(driver.RowTypeConverters);
        Assert.Same(EmptyDefaultValueConverter.Instance, driver.DefaultValueConverter);
    }
}
