using System.Text.RegularExpressions;
using Datasources;

namespace Datasources.Tests;

public class DefaultValueConverterTests
{
    [Fact]
    public void ConvertsUtcNowAndNewId()
    {
        var converter = new DefaultValueConverter(new DefaultValueOptions
        {
            NewId = () => "gen_random_uuid()",
            Now = () => "LOCALTIMESTAMP",
            UtcNow = () => "(NOW() AT TIME ZONE 'UTC')",
            BooleanOn = "TRUE",
            BooleanOff = "FALSE",
            Hex = arg => $"'\\x{arg}'",
            HexReverse = new Regex(@"^'\\x(.*)'$"),
        });
        Assert.Equal("(NOW() AT TIME ZONE 'UTC')", converter.Convert("datetime", "UtcNow"));
        Assert.Equal("gen_random_uuid()", converter.Convert("uuid", "NewId"));
        Assert.Equal("TRUE", converter.Convert("boolean", "true"));
    }
}
