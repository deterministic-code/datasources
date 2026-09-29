using Datasources;
using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace Datasources.Sqlite.Tests;

public class SqliteCommandIntegrationTests
{
    public SqliteCommandIntegrationTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class NameValue
    {
        public string name { get; set; } = "";
        public string value { get; set; } = "";
    }

    sealed class Item
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class User
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public string email { get; set; } = "";
        public string contact_name1 { get; set; } = "";
        public string contact_name2 { get; set; } = "";
        public List<Contact> contacts { get; set; } = [];

        public sealed class Contact
        {
            public string name { get; set; } = "";
        }
    }

    static TransformMapper<User, User> UserContactsMapper() =>
        new(
            row =>
            {
                row.contacts =
                [
                    new User.Contact { name = row.contact_name1 },
                    new User.Contact { name = row.contact_name2 }
                ];
                return row;
            },
            user =>
            {
                user.contact_name1 = user.contacts.ElementAtOrDefault(0)?.name ?? "";
                user.contact_name2 = user.contacts.ElementAtOrDefault(1)?.name ?? "";
                return user;
            });

    [Fact]
    public async Task ReturnStatementSelectsScalar()
    {
        var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = ":memory:" });
        await conn.Open();
        var command = new SqliteCommand(conn);
        var parameters = new[] { new Parameter("x", -3) };
        var response = await command.Execute(new ReturnStatement("abs", parameters), parameters);
        Assert.Equal(new Value.Int(3), response.Result);
    }

    [Fact]
    public async Task RunRawSql()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)");
        await command.Execute("INSERT INTO items (id, name) VALUES (1, 'ada')");
        await using var reader = await command.ExecuteReader("SELECT id, name FROM items");
        Row? row = null;
        if (await reader.Next())
            row = reader.Current;
        Assert.Equal(new Value.Int(1), row!["id"]);
        Assert.Equal(new Value.Text("ada"), row["name"]);
    }

    [Fact]
    public async Task RunExecuteReaderAndReturnNameValue()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute("CREATE TABLE settings (name TEXT, value TEXT)");
        await command.Execute(
            "INSERT INTO settings (name, value) VALUES ($name, $value)",
            [new Parameter("name", "theme"), new Parameter("value", "dark")]);
        await using var reader = await command.ExecuteReader("SELECT name, value FROM settings");
        var converter = new RowTypeConverter<NameValue>();
        var items = new List<NameValue>();
        while (await reader.Next())
            items.Add(converter.Convert(reader.Current!));
        Assert.Equal("theme", items[0].name);
        Assert.Equal("dark", items[0].value);
    }

    [Fact]
    public async Task ExecuteInsertUpdateAndDelete()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute("CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)");
        await command.Execute(
            "INSERT INTO items (id, name) VALUES ($id, $name)",
            [new Parameter("id", 1), new Parameter("name", "a")]);
        await command.Execute(
            "UPDATE items SET name = $name WHERE id = $id",
            [new Parameter("id", 1), new Parameter("name", "A")]);
        var converter = new RowTypeConverter<Item>();
        Item? updated = null;
        {
            await using var reader = await command.ExecuteReader(
                "SELECT id, name FROM items WHERE id = $id",
                [new Parameter("id", 1)]);
            if (await reader.Next())
                updated = converter.Convert(reader.Current!);
        }
        await command.Execute("DELETE FROM items WHERE id = $id", [new Parameter("id", 1)]);
        Item? gone = null;
        {
            await using var reader = await command.ExecuteReader(
                "SELECT id, name FROM items WHERE id = $id",
                [new Parameter("id", 1)]);
            if (await reader.Next())
                gone = converter.Convert(reader.Current!);
        }
        Assert.Equal(1, updated!.id);
        Assert.Equal("A", updated.name);
        Assert.Null(gone);
    }

    [Fact]
    public async Task ExecuteReaderMapsFlatContacts()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute("""
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT,
                email TEXT,
                contact_name1 TEXT,
                contact_name2 TEXT
            )
            """);
        await command.Execute(
            "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')");
        await using var reader = await command.ExecuteReader(
            "SELECT contact_name1, contact_name2, email, id, username FROM users");
        var converter = new RowTypeConverter<User>();
        var mapper = UserContactsMapper();
        User? user = null;
        while (await reader.Next())
            user = mapper.Map(converter.Convert(reader.Current!));
        Assert.Equal("bobby", user!.username);
        Assert.Equal("bob@asdf.com", user.email);
        Assert.Equal(["bob", "roberta"], user.contacts.Select(contact => contact.name));
    }

    [Fact]
    public async Task ReadMultipleResults()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await using var reader = await command.ExecuteReader("SELECT 1 AS id; SELECT 2 AS id");
        var first = new List<Row>();
        while (await reader.Next())
            first.Add(reader.Current!);
        var moved = await reader.NextResult();
        var second = new List<Row>();
        while (await reader.Next())
            second.Add(reader.Current!);
        var again = await reader.NextResult();
        Assert.True(moved);
        Assert.False(again);
        Assert.Equal(new Value.Int(1), first[0]["id"]);
        Assert.Equal(new Value.Int(2), second[0]["id"]);
    }

    [Fact]
    public async Task SharedSpecs()
    {
        var dialect = SqlSpec.Dialect.Load("sqlite", static statement => new SqliteStatementFactory().Transform(statement).GetText());
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml").Where(path => Path.GetFileName(path) != "schemas.yaml"))
            await SqlSpec.Run(path, dialect, target =>
            {
                var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = target });
                return new SqlSpec.Session(conn, new SqliteCommand(conn));
            }, () => new SqliteCommand(new SqliteConnection()));
    }
}
