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

    [Fact]
    public void MapsContactNames()
    {
        var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var contact = mapper.Map("Contact");
        var firstName = mapper.Map("FirstName");
        var emailAddress = mapper.Map("EmailAddress");
        Assert.Equal("contact", contact);
        Assert.Equal("first_name", firstName);
        Assert.Equal("email_address", emailAddress);
    }

    [Fact]
    public void ReverseMapsContactNames()
    {
        var mapper = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var contact = mapper.ReverseMap("contact");
        var firstName = mapper.ReverseMap("first_name");
        var emailAddress = mapper.ReverseMap("email_address");
        Assert.Equal("Contact", contact);
        Assert.Equal("FirstName", firstName);
        Assert.Equal("EmailAddress", emailAddress);
    }
}
