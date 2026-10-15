using Datasources;

namespace Datasources.Tests;

public class DeterministicTypeConverterTests
{
    [Fact]
    public void MapsNameAndSizedString()
    {
        var sqlite = new DeterministicTypeConverter(
            new Dictionary<string, string> { ["string"] = "TEXT", ["uuid"] = "TEXT" },
            new SpecFieldRules
            {
                DialectName = "sqlite",
                StringUnlimited = "TEXT",
                StringSized = n => $"VARCHAR({n})",
            });
        Assert.Equal("TEXT", sqlite.Convert("string"));
        Assert.Equal("VARCHAR(64)", sqlite.Convert(new SpecField { Type = "string", Size = 64 }));
    }
}
