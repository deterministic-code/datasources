using Datasources;
using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
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
        public long id { get; set; }
        public string name { get; set; } = "";
    }

    record User
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public string email { get; set; } = "";
        public string contact_name1 { get; set; } = "";
        public string contact_name2 { get; set; } = "";
        public List<Contact> contacts { get; set; } = [];

        public record Contact
        {
            public string name { get; set; } = "";
        }
    }

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
    public async Task ReadsBooleanAsBoolAndLeavesInteger()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await SchemaSpec.Apply(connection, "flags");
        await command.Execute(Native.N("INSERT INTO flags (enabled, count) VALUES (1, 1)"));
        await using var reader = await command.ExecuteReader(Native.N("SELECT enabled, count, 1 AS one FROM flags"));
        Assert.True(await reader.Next());
        var row = reader.Current!;
        Assert.Equal(new Value.Bool(true), row["enabled"]);
        Assert.Equal(new Value.Int(1), row["count"]);
        Assert.Equal(new Value.Int(1), row["one"]);
    }

    [Fact]
    public async Task RunExecuteReaderOnASimpleList()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        await SchemaSpec.Apply(command.Connection, "items");
        await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (1, 'ada')"));
        await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (2, 'grace')"));
        await command.Execute(Native.N("INSERT INTO items (id, name) VALUES (3, 'bea')"));
        await using var reader = await command.ExecuteReader("SELECT id, name FROM items");
        var rows = await reader.Read().ToListAsync();
        Assert.Equal(3, rows.Count);
        Assert.Equal(new Value.Int(1), rows[0]["id"]);
        Assert.Equal(new Value.Text("ada"), rows[0]["name"]);
        Assert.Equal(new Value.Int(2), rows[1]["id"]);
        Assert.Equal(new Value.Text("grace"), rows[1]["name"]);
        Assert.Equal(new Value.Int(3), rows[2]["id"]);
        Assert.Equal(new Value.Text("bea"), rows[2]["name"]);
    }

    [Fact]
    public async Task ReadAndApplyCaseMappings()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        await SchemaSpec.Apply(command.Connection, "case-contacts");
        await command.Execute(Native.N("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')"));
        await using var reader = await command.ExecuteReader("SELECT user_id, first_name FROM contacts");
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var mapped = await reader.Read().Select(source =>
            new Row(source.ToDictionary(cell => casing.ReverseMap(cell.Key), cell => cell.Value))).ToListAsync();
        Assert.Equal(new Value.Int(1), mapped[0]["UserId"]);
        Assert.Equal(new Value.Text("Ada"), mapped[0]["FirstName"]);
    }

    [Fact]
    public async Task ExecuteReaderThenApplyFlatToNestedMapper()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        await SchemaSpec.Apply(command.Connection, "flat-users");
        await command.Execute(Native.N("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')"));
        await command.Execute(Native.N("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')"));
        await using var reader = await command.ExecuteReader("SELECT id, username, contact_name FROM users");
        var rows = await reader.Read().ToListAsync();
        var mapper = new FlatToNestedMapper(new TypeMap
        {
            Key = "id",
            Fields = new Dictionary<string, TypeMapField>
            {
                ["id"] = "id",
                ["username"] = "username",
                ["contacts"] = new TypeMapGroup
                {
                    Key = "contact_name",
                    Fields = new Dictionary<string, TypeMapField> { ["name"] = "contact_name" },
                },
            },
        });
        var nested = mapper.Map(rows);
        Assert.Equal(new MappedValue.Cell(new Value.Int(1)), Assert.Single(nested)["id"]);
        Assert.Equal(new MappedValue.Cell(new Value.Text("bobby")), nested[0]["username"]);
        Assert.Equal(new MappedValue.Cell(new Value.Text("bob")), ((MappedValue.Many)nested[0]["contacts"]).Items[0]["name"]);
        Assert.Equal(new MappedValue.Cell(new Value.Text("roberta")), ((MappedValue.Many)nested[0]["contacts"]).Items[1]["name"]);
    }

    [Fact]
    public async Task RunExecuteReaderAndReturnNameValues()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await SchemaSpec.Apply(connection, "settings");
        await command.Execute(Native.N("INSERT INTO settings (name, value) VALUES ('theme', 'dark')"));
        await command.Execute(Native.N("INSERT INTO settings (name, value) VALUES ('lang', 'en')"));
        await command.Execute(Native.N("INSERT INTO settings (name, value) VALUES ('size', 'lg')"));
        await using var reader = await command.ExecuteReader("SELECT name, value FROM settings");
        var converter = new MetaDataRowTypeConverter<NameValue>();
        var items = await reader.Read().Select(row => converter.Convert(row)).ToListAsync();
        Assert.Equal("theme", items[0].name);
        Assert.Equal("dark", items[0].value);
        Assert.Equal("lang", items[1].name);
        Assert.Equal("en", items[1].value);
        Assert.Equal("size", items[2].name);
        Assert.Equal("lg", items[2].value);
    }

    [Fact]
    public async Task ExecuteInsertReadUpdateAndDelete()
    {
        var ds = Datasource.Default;
        await using var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await SchemaSpec.Apply(connection, "items");
        await command.Execute(
            "INSERT INTO items (id, name) VALUES ($id, $name)",
            [new Parameter("id", 1), new Parameter("name", "a")]);
        var converter = new MetaDataRowTypeConverter<Item>();
        await using var createdReader = await command.ExecuteReader(
            "SELECT id, name FROM items WHERE id = $id",
            [new Parameter("id", 1)]);
        var created = await createdReader.Read().Select(row => converter.Convert(row)).ToListAsync();
        var response = await command.Execute(
            "UPDATE items SET name = $name WHERE id = $id",
            [new Parameter("id", 1), new Parameter("name", "A")]);
        await using var updatedReader = await command.ExecuteReader(
            "SELECT id, name FROM items WHERE id = $id",
            [new Parameter("id", 1)]);
        var updated = await updatedReader.Read().Select(row => converter.Convert(row)).ToListAsync();
        await command.Execute("DELETE FROM items WHERE id = $id", [new Parameter("id", 1)]);
        await using var goneReader = await command.ExecuteReader(
            "SELECT id, name FROM items WHERE id = $id",
            [new Parameter("id", 1)]);
        var gone = await goneReader.Read().Select(row => converter.Convert(row)).ToListAsync();
        Assert.Equal(1, created[0].id);
        Assert.Equal("a", created[0].name);
        Assert.Equal(1, response.RowsAffected);
        Assert.Equal(1, updated[0].id);
        Assert.Equal("A", updated[0].name);
        Assert.Empty(gone);
    }

    [Fact]
    public async Task ExecuteReaderMapsFlatContacts()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        await SchemaSpec.Apply(command.Connection, "flat-contact-users");
        await command.Execute(Native.N(
            "INSERT INTO users (id, username, email, contact_name1, contact_name2) VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')"));
        await using var reader = await command.ExecuteReader(
            "SELECT contact_name1, contact_name2, email, id, username FROM users");
        var converter = new MetaDataRowTypeConverter<User>();
        TransformMapper<User, User> UserContactsMapper() =>
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
        var mapper = UserContactsMapper();
        var users = await reader.Read().Select(row => mapper.Map(converter.Convert(row))).ToListAsync();
        Assert.Equal("bobby", users[0].username);
        Assert.Equal("bob@asdf.com", users[0].email);
        Assert.Equal(["bob", "roberta"], users[0].contacts.Select(contact => contact.name));
    }

    [Fact]
    public async Task ReadMultipleResults()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        await using var reader = await command.ExecuteReader(Native.N("SELECT 1 AS id; SELECT 2 AS id"));
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
    public async Task LogsAfterCommandRowsForASelect()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        var lines = new List<string>();
        command.AddMiddleware(new ConsoleCommandMiddleware(lines.Add, () => DateTimeOffset.Parse("2026-09-22T00:00:00Z"), new Dictionary<string, string> { ["isVerbose"] = "true" }));
        await using var reader = await command.ExecuteReader(Native.N("SELECT 1 AS id"));
        Assert.True(await reader.Next());
        Assert.Contains(lines, line => line.Contains("[AfterCommand]-[{id: 1}]"));
    }

    [Fact]
    public async Task SharedSpecs()
    {
        var dialect = SqlSpec.Dialect.Load("sqlite", static statement =>
            new StatementTextRenderer(t => new SqliteStatementRenderer(t)).Render(statement));
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml").Where(path => Path.GetFileName(path) != "schemas.yaml"))
            await SqlSpec.Run(path, dialect, target =>
            {
                var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = target });
                return new SqlSpec.Session(conn, new SqliteCommand(conn));
            }, () => new SqliteCommand(new SqliteConnection()));
    }
}
