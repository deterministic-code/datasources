using Datasources;

namespace Datasources.Tests;

public class PluralizerMapperTests
{
    [Fact]
    public void PluralizesAndLowercasesTheTableName()
    {
        var mapper = new PluralizerMapper();
        Assert.Equal("Users", mapper.Map("User"));
        Assert.Equal("User", mapper.ReverseMap("Users"));
    }

    [Fact]
    public void MapsContacts()
    {
        var mapper = new PluralizerMapper();
        var contact = mapper.Map("Contact");
        var address = mapper.Map("Address");
        Assert.Equal("Contacts", contact);
        Assert.Equal("Addresses", address);
    }

    [Fact]
    public void ReverseMapsContacts()
    {
        var mapper = new PluralizerMapper();
        var contact = mapper.ReverseMap("Contacts");
        var address = mapper.ReverseMap("Addresses");
        Assert.Equal("Contact", contact);
        Assert.Equal("Address", address);
    }

    [Fact]
    public void LowercasesThenPluralizes()
    {
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var pluralizer = new PluralizerMapper();
        var contact = pluralizer.Map(casing.Map("Contact"));
        var address = pluralizer.Map(casing.Map("Address"));
        var contactName = casing.ReverseMap(pluralizer.ReverseMap(contact));
        var addressName = casing.ReverseMap(pluralizer.ReverseMap(address));
        Assert.Equal("contacts", contact);
        Assert.Equal("addresses", address);
        Assert.Equal("Contact", contactName);
        Assert.Equal("Address", addressName);
    }
}
