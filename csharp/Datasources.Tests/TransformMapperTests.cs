using Datasources;

namespace Datasources.Tests;

public class TransformMapperTests
{
    public record User(int Id, string Name);
    public record Contacts(string Email, string Phone);
    public record Nested(User User, Contacts Contacts);
    public record Flat(int UserId, string UserName, string ContactEmail, string ContactPhone);
    public record Contact(string Name, string Email);
    public record ContactRow(string ContactName, string ContactEmail);

    [Fact]
    public void FlattensAndUnflattensUserAndContacts()
    {
        var nested = new Nested(new User(1, "Ada"), new Contacts("ada@x.dev", "555"));
        var mapper = new TransformMapper<Nested, Flat>(
            map: n => new Flat(n.User.Id, n.User.Name, n.Contacts.Email, n.Contacts.Phone),
            reverseMap: f => new Nested(new User(f.UserId, f.UserName), new Contacts(f.ContactEmail, f.ContactPhone)));
        var flat = mapper.Map(nested);
        Assert.Equal(1, flat.UserId);
        Assert.Equal("Ada", flat.UserName);
        Assert.Equal("ada@x.dev", flat.ContactEmail);
        Assert.Equal("555", flat.ContactPhone);
        Assert.Equal(nested, mapper.ReverseMap(flat));
    }

    [Fact]
    public void PrefixesAContactField()
    {
        var mapper = new TransformMapper<string, string>(
            map: name => "contact_" + name,
            reverseMap: name => name["contact_".Length..]);
        var mapped = mapper.Map("email");
        var reversed = mapper.ReverseMap(mapped);
        Assert.Equal("contact_email", mapped);
        Assert.Equal("email", reversed);
    }

    [Fact]
    public void MapsAContactNameAndEmail()
    {
        var contact = new Contact("Ada", "ada@x.dev");
        var mapper = new TransformMapper<Contact, ContactRow>(
            map: item => new ContactRow(item.Name, item.Email),
            reverseMap: row => new Contact(row.ContactName, row.ContactEmail));
        var mapped = mapper.Map(contact);
        var reversed = mapper.ReverseMap(mapped);
        Assert.Equal(new ContactRow("Ada", "ada@x.dev"), mapped);
        Assert.Equal(contact, reversed);
    }
}
