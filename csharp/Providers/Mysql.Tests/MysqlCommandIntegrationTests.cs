using Datasources;
using Datasources.Mysql;
using Datasources.SqlSpecs;
using MySqlConnector;

namespace Datasources.Mysql.Tests;

public class MysqlCommandIntegrationTests
{
    sealed class Item
    {
        public long id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class ContactRow
    {
        public long id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class AddressRow
    {
        public long id { get; set; }
        public long contactId { get; set; }
        public string line1 { get; set; } = "";
    }

    [Fact]
    public async Task CreatePutsInsertedIdOnTheItem()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var isolated = await Isolate(url);
        try
        {
            var conn = new MysqlConnection(new MysqlConnectionOptions { Url = isolated.Url });
            await conn.Open();
            var command = new MysqlCommand(conn);
            await command.Execute(Native.N("CREATE TABLE items (id INTEGER PRIMARY KEY AUTO_INCREMENT, name TEXT)"));
            var native = await command.Execute(Native.N("INSERT INTO items (name) VALUES ('ada')"));
            Assert.Equal(Value.Null.Instance, native.Result);
            var fields = new Parameter[] { new("name", "bea") };
            var inserted = await command.Execute(new InsertStatement("items", fields) { Returning = "id" }, fields);
            Assert.NotEqual(Value.Null.Instance, inserted.Result);
            Assert.Equal(new Value.Int(2), inserted.Result);
            var repo = new CRUDRepository<Item>(conn, new CRUDRepositoryOptions<Item>
            {
                Ids = ["id"],
                Entity = "items",
            });
            var created = await repo.Create(new Item { name = "cee" });
            Assert.True(created.id > 0);
            var next = await repo.Create(new Item { name = "dee" });
            Assert.True(next.id > created.id);
        }
        finally
        {
            await isolated.Cleanup();
        }
    }

    [Fact]
    public async Task CreateThenChildInsertUsesTheParentId()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var isolated = await Isolate(url);
        try
        {
            var conn = new MysqlConnection(new MysqlConnectionOptions { Url = isolated.Url });
            await conn.Open();
            var command = new MysqlCommand(conn);
            await command.Execute(Native.N(
                "CREATE TABLE contacts (id INTEGER PRIMARY KEY AUTO_INCREMENT, name TEXT NOT NULL); CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTO_INCREMENT, contact_id INTEGER NOT NULL, line1 TEXT NOT NULL)"));
            var contacts = new CRUDRepository<ContactRow>(conn, new CRUDRepositoryOptions<ContactRow>
            {
                Ids = ["id"],
                Entity = "contacts",
                CreateProperties = ["name"],
            });
            var addresses = new CRUDRepository<AddressRow>(conn, new CRUDRepositoryOptions<AddressRow>
            {
                Ids = ["id"],
                Entity = "addresses",
                CreateProperties = ["contactId", "line1"],
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string> { ["contactId"] = "contact_id" }),
            });
            var created = await contacts.Create(new ContactRow { name = "ada" });
            await addresses.Create(new AddressRow { contactId = created.id, line1 = "12 Engine" });
            await using var reader = await command.ExecuteReader(Native.N("SELECT contact_id FROM addresses"));
            Assert.True(await reader.Next());
            Assert.Equal(new Value.Int(created.id), reader.Current!["contact_id"]);
        }
        finally
        {
            await isolated.Cleanup();
        }
    }

    [Fact]
    public async Task ExecuteReaderEnumeratesResultSets()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var isolated = await Isolate(url);
        try
        {
            var conn = new MysqlConnection(new MysqlConnectionOptions { Url = isolated.Url });
            await conn.Open();
            var command = new MysqlCommand(conn);
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
        finally
        {
            await isolated.Cleanup();
        }
    }

    [Fact]
    public async Task ReturnAndOutputParameters()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var isolated = await Isolate(url);
        try
        {
            var conn = new MysqlConnection(new MysqlConnectionOptions
            {
                Url = isolated.Url,
            });
            await conn.Open();
            var command = new MysqlCommand(conn);
            await FillOutputParameter(command);
            await CallProcedureReturnsTwoResults(command);
            await command.Execute(Native.N("""
                CREATE FUNCTION add_one(n INT) RETURNS INT DETERMINISTIC
                RETURN n + 1
                """));
            var n = new Parameter("n", 4);
            var ret = await command.Execute(new ReturnStatement("add_one", [n]), [n]);
            Assert.Equal(new Value.Int(5), ret.Result);
        }
        finally
        {
            await isolated.Cleanup();
        }
    }

    [Fact]
    public async Task SharedSpecs()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var dialect = SqlSpec.Dialect.Load("mysql", static statement =>
            new StatementTextRenderer(t => new MysqlStatementRenderer(t)).Render(statement));
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml").Where(path => Path.GetFileName(path) != "schemas.yaml"))
        {
            var isolated = await Isolate(url);
            try
            {
                await SqlSpec.Run(path, dialect, _ =>
                {
                    var conn = new MysqlConnection(new MysqlConnectionOptions
                    {
                        Url = isolated.Url,
                    });
                    return new SqlSpec.Session(conn, new MysqlCommand(conn));
                }, () => new MysqlCommand(new MysqlConnection()));
            }
            finally
            {
                await isolated.Cleanup();
            }
        }
    }

    static async Task FillOutputParameter(ICommand command)
    {
        await command.Execute(Native.N("""
            CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
            BEGIN
              SET result = start_n + 1;
            END
            """));
        var start = new Parameter("start_n", 4);
        var result = new Parameter("result", ParameterDirection.Output);
        var call = await command.Execute(new ExecuteStoredProcedureStatement("bump", [start, result]), [start, result]);
        Assert.Equal(new Value.Int(5), call.Parameters![1].Value);
    }

    static async Task CallProcedureReturnsTwoResults(ICommand command)
    {
        await command.Execute(Native.N("""
            CREATE PROCEDURE two_ids()
            BEGIN
              SELECT 1 AS id;
              SELECT 2 AS id;
            END
            """));
        await using var reader = await command.ExecuteReader(new ExecuteStoredProcedureStatement("two_ids", []));
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

    static async Task<(string Url, Func<Task> Cleanup)> Isolate(string baseUrl)
    {
        var name = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new MySqlConnection(MysqlConnectionString.ToConnectionString(baseUrl)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE DATABASE `{name}`";
            await create.ExecuteNonQueryAsync();
        }
        var isolated = ReplaceDatabase(baseUrl, name);
        return (isolated, async () =>
        {
            await using var drop = new MySqlConnection(MysqlConnectionString.ToConnectionString(baseUrl));
            await drop.OpenAsync();
            await using var cmd = drop.CreateCommand();
            cmd.CommandText = $"DROP DATABASE `{name}`";
            await cmd.ExecuteNonQueryAsync();
        });
    }

    static string ReplaceDatabase(string url, string name)
    {
        if (!url.StartsWith("mysql://", StringComparison.OrdinalIgnoreCase))
            return $"{url};Database={name}";
        var uri = new Uri(url);
        var builder = new UriBuilder(uri) { Path = name };
        return builder.Uri.ToString();
    }
}
