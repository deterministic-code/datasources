using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class DateTimeConverterTests
{
    [Fact]
    public void DateTime()
    {
        var converter = new DateTimeConverter();
        var native = converter.Convert("2020-01-02T03:04:05.000Z");
        Assert.Equal(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc), ((Value.DateTime)native).Item);
        Assert.Equal(new Value.Text("2020-01-02 03:04:05.000"), converter.ReverseConvert(native));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Int(1), converter.Convert(1L));
        Assert.Equal(new Value.Int(1), converter.ReverseConvert(1L));
    }
}
