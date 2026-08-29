using Datasources;
using Datasources.Oracle;

namespace Datasources.Oracle.Tests;

public class ConverterTableTests
{
    public ConverterTableTests() => TypeConverter.Reset();

    [Fact]
    public void RegistersEveryMappedType()
    {
        OracleConverters.Register();
        var mapping = TypeConverter.Mapping<Dictionary<string, Value>>("oracle");
        Assert.Equal("oracle", mapping.Source);
        foreach (var sqlType in OracleConverters.Types.Types.Keys)
        {
            var row = new Dictionary<string, Value> { ["col"] = Sample(sqlType) };
            var dest = mapping.To(row);
            Assert.NotNull(dest);
            Assert.NotNull(mapping.From(dest));
        }
    }

    static Value Sample(string sqlType)
    {
        var key = sqlType.ToUpperInvariant();
        if (key.Contains("BOOL") || key is "BIT" or "TINYINT(1)") return 1;
        if (key.Contains("DATE") || key.Contains("TIME")) return "2020-01-02T03:04:05.000Z";
        if (key.Contains("BLOB") || key.Contains("BINARY") || key is "BYTEA" or "RAW" or "VARBINARY") return "aGk=";
        if (key.Contains("UUID") || key is "UNIQUEIDENTIFIER" or "CHAR(36)") return "550e8400-e29b-41d4-a716-446655440000";
        if (key.Contains("NUMERIC") || key.Contains("DECIMAL") || key == "NUMBER") return "1.25";
        if (key.Contains("INT") || key.Contains("REAL") || key.Contains("FLOAT") || key.Contains("DOUBLE")) return 1;
        return "x";
    }
}
