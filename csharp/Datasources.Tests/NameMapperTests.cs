using Datasources;

namespace Datasources.Tests;

public class NameMapperTests
{
    [Fact]
    public void MapAndReverseMapAreTheName()
    {
        IMapper<string, string> mapping = new NameSourceDestinationMapper("sql", "csharp");
        Assert.Equal("user_accounts", mapping.Map("user_accounts"));
        Assert.Equal("UserAccounts", mapping.ReverseMap("UserAccounts"));
    }

    [Fact]
    public void SourceAndDestination()
    {
        var mapper = new NameSourceDestinationMapper("sql", "csharp");
        ISourceDestinationMapper labels = mapper;
        Assert.Equal("sql", labels.Source);
        Assert.Equal("csharp", labels.Destination);
        IMapper<string, string> mapping = mapper;
        Assert.Equal("user_accounts", mapping.Map("user_accounts"));
        Assert.Equal("UserAccounts", mapping.ReverseMap("UserAccounts"));
    }

    public record User(int Id, string Name);
    public record Contacts(string Email, string Phone);
    public record Nested(User User, Contacts Contacts);
    public record Flat(int UserId, string UserName, string ContactEmail, string ContactPhone);

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
}
