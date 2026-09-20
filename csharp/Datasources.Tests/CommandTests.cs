using Datasources;

namespace Datasources.Tests;

public class CommandTests
{
    sealed class NameValue
    {
        public string Name { get; set; } = "";
        public string Value { get; set; } = "";
    }

    sealed class Contact
    {
        public string Name { get; set; } = "";
    }

    sealed class User
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public string email { get; set; } = "";
        public string contact_name1 { get; set; } = "";
        public string contact_name2 { get; set; } = "";
        public List<Contact> Contacts { get; set; } = [];
    }

    static TransformMapper<User, User> UserContactsMapper() =>
        new(
            row =>
            {
                row.Contacts =
                [
                    new Contact { Name = row.contact_name1 },
                    new Contact { Name = row.contact_name2 }
                ];
                return row;
            },
            user =>
            {
                user.contact_name1 = user.Contacts.ElementAtOrDefault(0)?.Name ?? "";
                user.contact_name2 = user.Contacts.ElementAtOrDefault(1)?.Name ?? "";
                return user;
            });

    [Fact]
    public async Task ExecuteReaderReturnsNameValueObjects()
    {
        var command = new MockCommand(new MockConnection())
        {
            Reader = new MockReader<Row>([new Row { ["Name"] = "theme", ["Value"] = "dark" }])
        };
        var reader = await command.ExecuteReader("SELECT Name, Value FROM settings");
        var converter = new RowTypeConverter<NameValue>();
        var items = new List<NameValue>();
        while (await reader.Next())
            items.Add(converter.Convert(reader.Current!));
        Assert.Equal("theme", items[0].Name);
        Assert.Equal("dark", items[0].Value);
    }

    [Fact]
    public async Task ExecuteInsertUpdateDelete()
    {
        var command = new MockCommand(new MockConnection());
        await command.Execute("insert", [new Parameter("id", 1), new Parameter("name", "a")]);
        await command.Execute("update", [new Parameter("id", 1), new Parameter("name", "A")]);
        await command.Execute("delete", [new Parameter("id", 1)]);
        Assert.Equal(["insert", "update", "delete"], command.ExecuteCalls.Select(call => call.Text));
        Assert.Equal(["id", "name"], command.ExecuteCalls[0].Parameters!.Select(parameter => parameter.Name));
    }

    [Fact]
    public async Task ExecuteReaderMapsFlatContacts()
    {
        var command = new MockCommand(new MockConnection())
        {
            Reader = new MockReader<Row>([new Row
            {
                ["id"] = 1,
                ["username"] = "bobby",
                ["email"] = "bob@asdf.com",
                ["contact_name1"] = "bob",
                ["contact_name2"] = "roberta"
            }])
        };
        var reader = await command.ExecuteReader(
            "SELECT contact_name1, contact_name2, email, id, username FROM users");
        var converter = new RowTypeConverter<User>();
        var mapper = UserContactsMapper();
        User? user = null;
        while (await reader.Next())
            user = mapper.Map(converter.Convert(reader.Current!));
        Assert.Equal("bobby", user!.username);
        Assert.Equal("bob@asdf.com", user.email);
        Assert.Equal(["bob", "roberta"], user.Contacts.Select(contact => contact.Name));
    }
}
