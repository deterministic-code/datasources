using Datasources;

namespace Datasources.Tests;

public class DeterministicTypeConverterTests
{
    [Fact]
    public void MapsSizedStringAndRowVersion()
    {
        var sqlite = new DeterministicTypeConverter(
            new Dictionary<string, string> { ["string"] = "TEXT", ["uuid"] = "TEXT" },
            new SpecFieldRules
            {
                DialectName = "sqlite",
                StringUnlimited = "TEXT",
                StringSized = n => $"VARCHAR({n})",
            });
        Assert.Equal("VARCHAR(64)", sqlite.Convert(new SpecField { Type = "string", Size = 64 }));
        var sqlserver = new DeterministicTypeConverter(
            new Dictionary<string, string> { ["binary"] = "VARBINARY(MAX)" },
            new SpecFieldRules
            {
                DialectName = "sqlserver",
                StringUnlimited = "NVARCHAR(MAX)",
                StringSized = n => $"NVARCHAR({n})",
                OccBinary = "BINARY(8)",
                RowVersion = "ROWVERSION",
            });
        Assert.Equal(
            "ROWVERSION",
            sqlserver.Convert(new SpecField
            {
                Type = "binary",
                IsOptimisticConcurrency = true,
                UseNativeRowVersion = true,
            }));
    }
}
