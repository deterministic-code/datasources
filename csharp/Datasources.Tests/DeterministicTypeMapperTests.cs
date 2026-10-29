namespace Datasources.Tests;

public class DeterministicTypeMapperTests
{
    [Fact]
    public void MapsLanguageTypeNamesToTypesYamlNames()
    {
        var types = new DeterministicTypeMapper();
        Assert.Equal("boolean", types.Map(nameof(Boolean)));
        Assert.Equal("integer", types.Map(nameof(Int32)));
        Assert.Equal("uuid", types.Map(nameof(Guid)));
        Assert.Equal("datetime", types.Map(nameof(DateTime)));
        Assert.Equal("decimal", types.Map(nameof(Decimal)));
        Assert.Equal("binary", types.Map("Byte[]"));
        Assert.Equal("", types.Map(nameof(String)));
        Assert.Equal("Boolean", types.ReverseMap("boolean"));
        Assert.Equal("Int32", types.ReverseMap("integer"));
    }
}
