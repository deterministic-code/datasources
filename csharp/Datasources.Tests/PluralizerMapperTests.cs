using Datasources;

namespace Datasources.Tests;

public class PluralizerMapperTests
{
    [Fact]
    public void PluralizesAndLowercasesTheTableName()
    {
        var mapper = new PluralizerMapper();
        Assert.Equal("users", mapper.Map("User"));
        Assert.Equal("user", mapper.ReverseMap("users"));
    }

    [Fact]
    public void MapsContacts()
    {
        var mapper = new PluralizerMapper();
        var contact = mapper.Map("Contact");
        var address = mapper.Map("Address");
        Assert.Equal("contacts", contact);
        Assert.Equal("addresses", address);
    }

    [Fact]
    public void ReverseMapsContacts()
    {
        var mapper = new PluralizerMapper();
        var contact = mapper.ReverseMap("contacts");
        var address = mapper.ReverseMap("addresses");
        Assert.Equal("contact", contact);
        Assert.Equal("address", address);
    }
}
