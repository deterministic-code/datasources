using Datasources;
using Datasources.Builders;

namespace Datasources.Tests;

public class RepositoryTests
{
    public RepositoryTests() => TypeConverter.Reset();

    sealed class Item
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    static (Repository<Dictionary<string, object?>> Repo, MockCommand Command) Create(
        IReadOnlyList<string>? id = null,
        bool generated = false,
        IReadOnlyDictionary<string, string>? schema = null,
        IReadOnlyDictionary<string, IFieldConverter>? converters = null,
        IReadOnlyDictionary<string, string>? names = null)
    {
        TypeConverter.Use("memory", TypeConverter.IdentityProviderTypes());
        var connection = new MockConnection();
        var command = new MockCommand(connection);
        var options = new RepositoryOptions<Dictionary<string, object?>>
        {
            DatasourceName = "users",
            Connection = connection,
            CreateCommand = _ => command,
            Provider = "memory",
            Id = id ?? ["id"],
            Generated = generated,
            Schema = schema ?? new Dictionary<string, string> { ["id"] = "INTEGER", ["name"] = "TEXT" },
            Converters = converters,
            Names = names
        };
        return (new Repository<Dictionary<string, object?>>(options), command);
    }

    [Fact]
    public async Task GetBuildsSelectAndMapsRows()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader(new object?[]
        {
            new Dictionary<string, object?> { ["id"] = 1, ["name"] = "a" }
        });
        var rows = new List<Dictionary<string, object?>>();
        await foreach (var row in repo.Get())
            rows.Add(row);
        Assert.Single(rows);
        Assert.Equal(1, rows[0]["id"]);
        Assert.Equal("SELECT id, name FROM users", command.ExecuteReaderCalls[0].Text);
        Assert.Same(command.Connection, repo.Connection);
        command.Reader = new MockReader(new object?[]
        {
            new Dictionary<string, object?> { ["id"] = 1, ["name"] = "a" }
        });
        Assert.Equal(1, (await repo.GetSingle())!["id"]);
    }

    [Fact]
    public async Task GetUsesCallerQuery()
    {
        var (repo, command) = Create();
        command.Reader = new MockReader([]);
        var where = WhereBuilder.Equals("id", 9);
        await repo.GetSingle(StatementBuilder.CreateQuery("users", ["id"], where));
        Assert.Equal("SELECT id FROM users WHERE id = $id", command.ExecuteReaderCalls[0].Text);
        Assert.Equal(9, command.ExecuteReaderCalls[0].Parameters![0].Value);
        Assert.Null(await repo.GetSingle());
    }

    [Fact]
    public async Task AddUpdateDeleteUseBuilders()
    {
        var (repo, command) = Create();
        var obj = new Dictionary<string, object?> { ["id"] = 1, ["name"] = "a" };
        await repo.Add(obj);
        Assert.Equal("INSERT INTO users (id, name) VALUES ($id, $name)", command.ExecuteCalls[0].Text);
        await repo.Update(obj);
        Assert.Equal("UPDATE users SET name = $name WHERE id = $id", command.ExecuteCalls[1].Text);
        Assert.Equal("a", command.ExecuteCalls[1].Parameters![0].Value);
        Assert.Equal(1, command.ExecuteCalls[1].Parameters![1].Value);
        await repo.Delete(obj);
        Assert.Equal("DELETE FROM users WHERE id = $id", command.ExecuteCalls[2].Text);
    }

    [Fact]
    public async Task CompositeIdAndGeneratedAndOverrides()
    {
        var twice = new FieldConverter(v => v, v => v is int n ? n * 2 : v);
        var (repo, command) = Create(
            id: ["org_id", "user_id"],
            generated: true,
            schema: new Dictionary<string, string>
            {
                ["org_id"] = "INTEGER",
                ["user_id"] = "INTEGER",
                ["name"] = "TEXT"
            },
            converters: new Dictionary<string, IFieldConverter> { ["name"] = twice },
            names: new Dictionary<string, string> { ["name"] = "label" });
        await repo.Add(new Dictionary<string, object?>
        {
            ["org_id"] = 1, ["user_id"] = 2, ["label"] = 3
        });
        Assert.Equal("INSERT INTO users (name) VALUES ($name)", command.ExecuteCalls[0].Text);
        Assert.Equal(6, command.ExecuteCalls[0].Parameters![0].Value);
        await repo.Update(new Dictionary<string, object?>
        {
            ["org_id"] = 1, ["user_id"] = 2, ["label"] = 4
        });
        Assert.Contains("org_id = $org_id AND user_id = $user_id", command.ExecuteCalls[1].Text);
    }

    [Fact]
    public async Task MissingIdAndBadRowAndCtor()
    {
        Assert.Throws<DatasourceException>(() =>
            new Repository<Dictionary<string, object?>>(new RepositoryOptions<Dictionary<string, object?>>
            {
                DatasourceName = "users",
                Connection = new MockConnection(),
                CreateCommand = conn => new MockCommand(conn)
            }));
        var (repo, command) = Create();
        command.Reader = new MockReader(new object?[] { "nope" });
        await Assert.ThrowsAsync<DatasourceException>(async () =>
        {
            await foreach (var _ in repo.Get()) { }
        });
        await Assert.ThrowsAsync<DatasourceException>(() =>
            repo.Update(new Dictionary<string, object?> { ["name"] = "a" }));
        await Assert.ThrowsAsync<DatasourceException>(() =>
            repo.Delete(new Dictionary<string, object?> { ["name"] = "a" }));
        await Assert.ThrowsAsync<DatasourceException>(() =>
            repo.Update(new Dictionary<string, object?> { ["id"] = 1 }));
        TypeConverter.Use("memory", TypeConverter.SqlProviderTypes());
        var mapping = TypeConverter.Mapping<Item>("memory");
        var command2 = new MockCommand(new MockConnection());
        var typed = new Repository<Item>(new RepositoryOptions<Item>
        {
            DatasourceName = "users",
            Connection = command2.Connection,
            CreateCommand = _ => command2,
            Mapping = mapping
        });
        await typed.Add(new Item { Id = 1, Name = "a" });
        Assert.Contains("INSERT INTO users", command2.ExecuteCalls[0].Text);
        var where = WhereBuilder.Equals("name", "z");
        await typed.Delete(new Item { Name = "z" }, where);
        Assert.Equal("DELETE FROM users WHERE name = $name", command2.ExecuteCalls[1].Text);
    }
}
