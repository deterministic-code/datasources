using Datasources;

namespace Datasources.Tests;

public class TransformConverterMapperTests
{
    sealed class Wrap : TypeConverter<string, string>
    {
        public override string Convert(string from) => $"[{from}]";
        public override string ReverseConvert(string to) => to.Trim('[', ']');
    }

    [Fact]
    public void ConvertsBothWaysWithoutFilter()
    {
        var mapper = new TransformConverterMapper<string>(new Wrap());
        Assert.Equal("[ab]", mapper.Map("ab"));
        Assert.Equal("ab", mapper.ReverseMap("[ab]"));
    }

    [Fact]
    public void ConvertsWhenFilterIsTrue()
    {
        var mapper = new TransformConverterMapper<string>(new Wrap(), s => s.Contains("ab"));
        Assert.Equal("[ab]", mapper.Map("ab"));
        Assert.Equal("ab", mapper.ReverseMap("[ab]"));
    }

    [Fact]
    public void IsIdentityWhenFilterIsFalse()
    {
        var mapper = new TransformConverterMapper<string>(new Wrap(), s => s.Contains("ab"));
        Assert.Equal("a", mapper.Map("a"));
        Assert.Equal("[a]", mapper.ReverseMap("[a]"));
    }

    record Address(string Street, string City, string Country);
    record Contact(string Email, string Phone, Address Address);
    record User(string Username, string DisplayName, Contact Contact);

    sealed class UserFromCells : TypeConverter<User, string[]>
    {
        public override User Convert(string[] from) => from switch
        {
            [var username, var displayName, var email, var phone, var street, var city, var country] =>
                new User(username, displayName, new Contact(email, phone, new Address(street, city, country))),
            _ => throw new ArgumentException("expected 7 cells")
        };

        public override string[] ReverseConvert(User to) =>
        [
            to.Username, to.DisplayName, to.Contact.Email, to.Contact.Phone,
            to.Contact.Address.Street, to.Contact.Address.City, to.Contact.Address.Country
        ];
    }

    [Fact]
    public void ExpandsFlatCellsIntoNestedUser()
    {
        string[] cells =
        [
            "ada", "Ada Lovelace", "ada@example.com", "+44 20 7946 0958",
            "10 Downing St", "London", "UK"
        ];
        var expected = new User(
            "ada",
            "Ada Lovelace",
            new Contact("ada@example.com", "+44 20 7946 0958", new Address("10 Downing St", "London", "UK")));
        var mapper = new TransformConverterMapper<User, string[]>(new UserFromCells());
        Assert.Equal(expected, mapper.Map(cells));
        Assert.Equal(cells, mapper.ReverseMap(expected));
    }

    abstract record Payload;
    sealed record Flat(string[] Cells) : Payload;
    sealed record Account(
        int Id,
        string Username,
        string DisplayName,
        bool IsEnabled,
        DateTime Created,
        Contact Contact) : Payload;

    sealed class AccountFromCells : TypeConverter<Payload, Payload>
    {
        public override Payload Convert(Payload from) => from switch
        {
            Flat { Cells: [var id, var username, var displayName, var enabled, var created, var email, var phone, var street, var city, _] } =>
                new Account(
                    int.Parse(id),
                    username,
                    displayName,
                    enabled == "1",
                    DateTime.Parse(created, null, System.Globalization.DateTimeStyles.RoundtripKind),
                    new Contact(email, phone, new Address(street, city, ""))),
            _ => from
        };

        public override Payload ReverseConvert(Payload to) => to switch
        {
            Account account => new Flat(
            [
                account.Id.ToString(),
                account.Username,
                account.DisplayName,
                account.IsEnabled ? "1" : "0",
                (account.Created.Kind == DateTimeKind.Utc ? account.Created : account.Created.ToUniversalTime())
                    .ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'", System.Globalization.CultureInfo.InvariantCulture),
                account.Contact.Email,
                account.Contact.Phone,
                account.Contact.Address.Street,
                account.Contact.Address.City,
                ""
            ]),
            _ => to
        };
    }

    static bool IsEnabled(Payload payload) => payload switch
    {
        Flat { Cells: [_, _, _, "1", ..] } => true,
        Account { IsEnabled: true } => true,
        _ => false
    };

    [Fact]
    public void ConvertsTypesAndFiltersRows()
    {
        var ada = new Flat(
        [
            "7", "ada", "Ada Lovelace", "1", "2020-01-02T03:04:05.000Z",
            "ada@example.com", "+44", "10 Downing St", "London", "secret-note"
        ]);
        var guest = new Flat(
        [
            "0", "guest", "Guest", "0", "2020-01-01T00:00:00.000Z",
            "guest@example.com", "", "1 Road", "Paris", "ignore-me"
        ]);
        var expected = new Account(
            7,
            "ada",
            "Ada Lovelace",
            true,
            new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc),
            new Contact("ada@example.com", "+44", new Address("10 Downing St", "London", "")));
        var mapper = new TransformConverterMapper<Payload>(new AccountFromCells(), IsEnabled);
        Assert.Equal(expected, mapper.Map(ada));
        Assert.Equal(
            new[]
            {
                "7", "ada", "Ada Lovelace", "1", "2020-01-02T03:04:05.000Z",
                "ada@example.com", "+44", "10 Downing St", "London", ""
            },
            Assert.IsType<Flat>(mapper.ReverseMap(expected)).Cells);
        Assert.Same(guest, mapper.Map(guest));
        Assert.Same(guest, mapper.ReverseMap(guest));
    }
}
