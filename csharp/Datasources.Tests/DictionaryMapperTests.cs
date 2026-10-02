using Datasources;

namespace Datasources.Tests;

public class DictionaryMapperTests
{
    sealed class Account
    {
        public int Key { get; set; }
        public string Login { get; set; } = "";
    }

    [Fact]
    public void MapsKnownNamesAndPassesThroughTheRest()
    {
        var mapper = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["FirstName"] = "given_name",
            ["LastName"] = "family_name",
            ["Email"] = "email_address",
        });
        var firstName = mapper.Map("FirstName");
        var lastName = mapper.Map("LastName");
        var email = mapper.Map("Email");
        var phone = mapper.Map("Phone");
        var fromFirst = mapper.ReverseMap("given_name");
        var fromLast = mapper.ReverseMap("family_name");
        var fromEmail = mapper.ReverseMap("email_address");
        var fromPhone = mapper.ReverseMap("Phone");
        Assert.Equal("given_name", firstName);
        Assert.Equal("family_name", lastName);
        Assert.Equal("email_address", email);
        Assert.Equal("Phone", phone);
        Assert.Equal("FirstName", fromFirst);
        Assert.Equal("LastName", fromLast);
        Assert.Equal("Email", fromEmail);
        Assert.Equal("Phone", fromPhone);
    }

    [Fact]
    public void FirstKeyWinsOnReverse()
    {
        var mapper = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Home"] = "address",
            ["Work"] = "address",
        });
        var home = mapper.Map("Home");
        var work = mapper.Map("Work");
        var address = mapper.ReverseMap("address");
        Assert.Equal("address", home);
        Assert.Equal("address", work);
        Assert.Equal("Home", address);
    }

    [Fact]
    public async Task ReadReverseMapsColumnNames()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new ReadonlyRepository<Account>(connection, new ReadonlyRepositoryMappingOptions<Account>
        {
            Entity = "user",
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
            }),
        });
        command.Reader = new MockReader<Row>([new Row { ["id"] = 7, ["username"] = "ada" }]);
        var row = await repo.ReadSingle();
        Assert.NotNull(row);
        Assert.Equal(7, row.Key);
        Assert.Equal("ada", row.Login);
        Assert.Equal("SELECT id, username FROM user", command.ExecuteReaderCalls[0].Text);
    }
}
