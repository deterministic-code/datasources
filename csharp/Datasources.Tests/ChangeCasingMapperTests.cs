using Datasources;

namespace Datasources.Tests;

public class ChangeCasingMapperTests
{
    [Fact]
    public void MapsPascalToSnakeAndBack()
    {
        var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        Assert.Equal("user_type", mapper.Map("UserType"));
        Assert.Equal("id", mapper.Map("Id"));
        Assert.Equal("UserType", mapper.ReverseMap("user_type"));
        Assert.Equal("Id", mapper.ReverseMap("id"));
    }
}
