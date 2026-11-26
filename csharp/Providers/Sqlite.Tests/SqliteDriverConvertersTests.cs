using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteDriverConvertersTests
{
    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<TypeMap>(new SqliteDriver().TypeConverter);
        ConversionMatrix.Types("sqlite", types.Convert, types.Convert, types.ReverseConvert);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        var defaults = new SqliteDriver().DefaultValueConverter;
        ConversionMatrix.Defaults("sqlite", defaults);
        Assert.Contains("randomblob", defaults.Convert("uuid", "NewId"));
        Assert.Equal("NewId", defaults.ReverseConvert("uuid", defaults.Convert("uuid", "NewId")!));
    }

    [Fact]
    public void ConvertsCells()
    {
        ConversionMatrix.Cells("sqlite", new SqliteDriver().RowTypeConverters);
    }
}
