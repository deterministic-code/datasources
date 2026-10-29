namespace Datasources.Tests;

public class DeterministicTypeMapperTests
{
    [Fact]
    public void MapsLanguageTypeNamesToTypesYamlNames()
    {
        var types = new DeterministicTypeMapper();
        Assert.Equal("boolean", types.Map(nameof(Boolean)));
        Assert.Equal("smallinteger", types.Map(nameof(SByte)));
        Assert.Equal("smallinteger", types.Map(nameof(Int16)));
        Assert.Equal("unsignedsmallinteger", types.Map(nameof(Byte)));
        Assert.Equal("unsignedsmallinteger", types.Map(nameof(UInt16)));
        Assert.Equal("integer", types.Map(nameof(Int32)));
        Assert.Equal("unsignedinteger", types.Map(nameof(UInt32)));
        Assert.Equal("biginteger", types.Map(nameof(Int64)));
        Assert.Equal("unsignedbiginteger", types.Map(nameof(UInt64)));
        Assert.Equal("uuid", types.Map(nameof(Guid)));
        Assert.Equal("datetime", types.Map(nameof(DateTime)));
        Assert.Equal("decimal", types.Map(nameof(Decimal)));
        Assert.Equal("binary", types.Map("Byte[]"));
        Assert.Equal("", types.Map(nameof(String)));
        Assert.Equal("Boolean", types.ReverseMap("boolean"));
        Assert.Equal("Int16", types.ReverseMap("smallinteger"));
        Assert.Equal("UInt16", types.ReverseMap("unsignedsmallinteger"));
        Assert.Equal("Int32", types.ReverseMap("integer"));
        Assert.Equal("UInt32", types.ReverseMap("unsignedinteger"));
        Assert.Equal("Int64", types.ReverseMap("biginteger"));
        Assert.Equal("UInt64", types.ReverseMap("unsignedbiginteger"));
        Assert.Equal("Guid", types.ReverseMap("uuid"));
        Assert.Equal("DateTime", types.ReverseMap("datetime"));
        Assert.Equal("Decimal", types.ReverseMap("decimal"));
        Assert.Equal("Byte[]", types.ReverseMap("binary"));
        Assert.Equal("", types.ReverseMap("string"));
    }
}
