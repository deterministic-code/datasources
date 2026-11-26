using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlDriverConvertersTests
{
    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<MysqlDriver.TypeMap>(new MysqlDriver().TypeConverter);
        ConversionMatrix.Types("mysql", types.Convert, types.Convert, types.ReverseConvert);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        var defaults = new MysqlDriver().DefaultValueConverter;
        ConversionMatrix.Defaults("mysql", defaults);
        Assert.Contains("00000000", defaults.Convert("uuid", "Empty"));
    }

    [Fact]
    public void ConvertsCells()
    {
        ConversionMatrix.Cells("mysql", new MysqlDriver().RowTypeConverters);
    }
}
