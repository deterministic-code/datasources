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
            ["Key"] = "id",
            ["Login"] = "username",
        });
        Assert.Equal("id", mapper.Map("Key"));
        Assert.Equal("Key", mapper.ReverseMap("id"));
        Assert.Equal("other", mapper.Map("other"));
        Assert.Equal("other", mapper.ReverseMap("other"));
    }

    [Fact]
    public async Task ReadReverseMapsColumnNames()
    {
        var connection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(connection);
        connection.Command = command;
        var repo = new ReadonlyRepository<Account>(connection, new ReadonlyRepositoryOptions<Account>
        {
            Entity = "user",
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
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
