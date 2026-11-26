using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresDriverConvertersTests
{
    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<TypeMap>(new PostgresDriver().TypeConverter);
        ConversionMatrix.Types("postgres", types.Convert, types.Convert, types.ReverseConvert);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        ConversionMatrix.Defaults("postgres", new PostgresDriver().DefaultValueConverter);
    }

    [Fact]
    public void ConvertsCells()
    {
        ConversionMatrix.Cells("postgres", new PostgresDriver().RowTypeConverters);
    }
}
